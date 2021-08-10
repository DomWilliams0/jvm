use crate::alloc::{vmref_into_raw, VmRef};
use crate::class::{FunctionArgs, Method};
use crate::interpreter::InterpreterError;
use crate::jni::sys::JNIEnv;

use log::*;

use region::{Allocation, Protection};
use smallvec::SmallVec;

use crate::types::{DataType, DataValue};
use itertools::repeat_n;
use std::io::Cursor;
use std::mem::transmute_copy;

const BACKING_CAPACITY: usize = 4096 * 16;

struct ThunkBacking {
    backing: Allocation,
    /// Offset into backing where next thunk can start
    next: usize,
    // TODO support freeing of thunks when a class is unloaded, and reuse it
}

#[derive(Default)]
pub struct NativeThunks {
    backing: Vec<ThunkBacking>,
}

/// A handle to the allocation in a [NativeThunks], which owns the memory on our behalf
#[derive(Clone)]
pub struct NativeThunkHandle(*mut u8, usize);

impl NativeThunks {
    pub fn new_for_method(
        &mut self,
        method: &Method,
        fn_ptr: usize,
    ) -> Result<NativeThunkHandle, InterpreterError> {
        let handle = self.allocate()?;

        emit_thunk(method.args(), handle.as_slice(), fn_ptr)?;
        Ok(handle)
    }

    fn allocate(&mut self) -> Result<NativeThunkHandle, InterpreterError> {
        let alloc = self.backing.iter_mut().find_map(|b| b.allocate());
        if let Some((ptr, len)) = alloc {
            return Ok(NativeThunkHandle(ptr, len));
        }

        // allocate new space
        let mut allocation = region::alloc(BACKING_CAPACITY, Protection::READ_WRITE_EXECUTE)?;

        unsafe {
            let slice =
                std::slice::from_raw_parts_mut::<u8>(allocation.as_mut_ptr(), allocation.len());
            slice.fill(0xcc);
        }

        self.backing.push(ThunkBacking {
            backing: allocation,
            next: 0,
        });

        let backing = self.backing.last_mut().unwrap(); // just added
        let (ptr, len) = backing
            .allocate()
            .expect("new backing allocation is too small");
        Ok(NativeThunkHandle(ptr, len))
    }
}

impl NativeThunkHandle {
    fn as_slice(&self) -> &'static mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.0, self.1) }
    }

    /// Returns *RAW UNINTERPRETED RETURN VALUE*
    pub fn invoke<T>(
        &self,
        jnienv: *const JNIEnv,
        jclass_or_object: VmRef<T>,
        args: FunctionArgs,
    ) -> u64 {
        self.invoke_inner(jnienv, vmref_into_raw(jclass_or_object) as *const (), args)
    }

    fn invoke_inner(
        &self,
        jnienv: *const JNIEnv,
        jclass_or_object: *const (),
        args: FunctionArgs,
    ) -> u64 {
        let mut native_args: SmallVec<[u64; 8]> =
            smallvec::smallvec![jnienv as u64, jclass_or_object as u64];

        native_args.extend(repeat_n(0, args.len()));

        for (arg, arg_out) in args.take_all().zip(&mut native_args[2..]) {
            // squidge all arg types into a u64, which will be read correctly by thunk
            let mut val = 0u64;

            unsafe {
                match arg {
                    DataValue::Boolean(arg) => val = transmute_copy(&arg),
                    DataValue::Byte(arg) => val = transmute_copy(&arg),
                    DataValue::Short(arg) => val = transmute_copy(&arg),
                    DataValue::Int(arg) => val = transmute_copy(&arg),
                    DataValue::Long(arg) => val = transmute_copy(&arg),
                    DataValue::Char(arg) => val = transmute_copy(&arg),
                    DataValue::Float(arg) => val = transmute_copy(&arg),
                    DataValue::Double(arg) => val = transmute_copy(&arg),
                    DataValue::Reference(arg) => val = vmref_into_raw(arg) as u64,
                    DataValue::VmDataClass(_) | DataValue::ReturnAddress(_) => unreachable!(),
                }
            }

            *arg_out = val;
        }

        let ret: u64;
        unsafe {
            trace!("calling thunk at {:?}", self.0);
            asm!(
                "call {thunk}",
                "mov {ret}, rax",
                thunk = in(reg) self.0,
                ret = out(reg) ret,
                in("rax") native_args.as_ptr(),
                options(nostack, nomem)
            );
        }

        ret
    }
}

impl ThunkBacking {
    fn allocate(&mut self) -> Option<(*mut u8, usize)> {
        const SIZE: usize = 1024;
        let new_end = self.next + SIZE;

        if new_end >= self.backing.len() {
            None
        } else {
            let start = std::mem::replace(&mut self.next, new_end);
            let len = SIZE;
            // TODO size depends on number of args to pass

            let start = unsafe { self.backing.as_mut_ptr::<u8>().add(start) };
            Some((start, len))
        }
    }
}

enum IntRegister {
    Rdx,
    Rcx,
    R8,
    R9,
}

enum IntRegisterSize {
    U8,
    U16,
    U32,
    U64,
}

fn emit_thunk(args: &[DataType], out: &mut [u8], fn_ptr: usize) -> std::io::Result<()> {
    let cursor = Cursor::new(out);
    r#impl::emit_thunk(args, cursor, fn_ptr)
}

#[cfg(all(target_arch = "x86_64", unix))]
mod r#impl {
    use crate::interpreter::native::{IntRegister, IntRegisterSize};
    use crate::types::{DataType, PrimitiveDataType};
    use std::array::IntoIter;
    use std::io::{Cursor, Write};

    pub fn emit_thunk(
        args: &[DataType],
        mut cursor: Cursor<&mut [u8]>,
        fn_ptr: usize,
    ) -> std::io::Result<()> {
        // nop
        cursor.write_all(&[0x90])?;

        // mov    rdi, [rax]    ; jni env
        // mov    rsi, [rax+8h] ; jobject/jclass
        cursor.write_all(&[0x48, 0x8B, 0x38, 0x48, 0x8B, 0x70, 0x08])?;

        if !args.is_empty() {
            // add    rax,0x10
            cursor.write_all(&[0x48, 0x83, 0xC0, 0x10])?;
        }

        let mut int_registers = IntoIter::new([
            IntRegister::Rdx,
            IntRegister::Rcx,
            IntRegister::R8,
            IntRegister::R9,
        ]);
        // TODO float registers

        use PrimitiveDataType::*;
        for (i, arg) in args.iter().enumerate() {
            let int_type = match arg {
                DataType::Primitive(Float) | DataType::Primitive(Double) => {
                    unimplemented!("float args")
                }

                DataType::Primitive(Boolean) | DataType::Primitive(Byte) => IntRegisterSize::U8,
                DataType::Primitive(Char) | DataType::Primitive(Short) => IntRegisterSize::U16,
                DataType::Primitive(Int) => IntRegisterSize::U32,
                DataType::Primitive(Long) | DataType::Reference(_) => IntRegisterSize::U64,
                DataType::ReturnAddress => unreachable!(),
            };

            let register = int_registers.next().expect("TODO: stack spillover");
            register.emit_store(int_type, i, &mut cursor)?;
        }

        // mov  r11, fn_ptr
        // jmp  r11
        let callee_addr = fn_ptr.to_le_bytes();
        cursor.write_all(&[0x49, 0xBB])?;
        cursor.write_all(&callee_addr)?;
        cursor.write_all(&[0x41, 0xFF, 0xE3])?;

        // ud2
        cursor.write_all(&[0x0F, 0x0B])?;
        Ok(())
    }

    impl IntRegister {
        fn emit_store(
            &self,
            size: IntRegisterSize,
            arg_i: usize,
            mut cursor: impl Write,
        ) -> std::io::Result<()> {
            use IntRegister::*;
            use IntRegisterSize::*;
            let offset = {
                let mul = arg_i * 8;
                debug_assert!(mul < 256); // won't have enough int
                mul as u8
            };
            match (self, size) {
                //    mov    dl,BYTE PTR [rax+$OFFSET]
                (Rdx, U8) => cursor.write_all(&[0x8a, 0x50, offset]),
                //    mov    dx,WORD PTR [rax+$OFFSET]
                (Rdx, U16) => cursor.write_all(&[0x66, 0x8b, 0x50, offset]),
                //    mov    edx,DWORD PTR [rax+$OFFSET]
                (Rdx, U32) => cursor.write_all(&[0x8b, 0x50, offset]),
                //    mov    rdx,QWORD PTR [rax+$OFFSET]
                (Rdx, U64) => cursor.write_all(&[0x48, 0x8b, 0x50, offset]),

                //    mov    cl,BYTE PTR [rax+$OFFSET]
                (Rcx, U8) => cursor.write_all(&[0x8a, 0x48, offset]),
                //    mov    cx,WORD PTR [rax+$OFFSET]
                (Rcx, U16) => cursor.write_all(&[0x66, 0x8b, 0x48, offset]),
                //    mov    ecx,DWORD PTR [rax+$OFFSET]
                (Rcx, U32) => cursor.write_all(&[0x8b, 0x48, offset]),
                //    mov    rcx,QWORD PTR [rax+$OFFSET]
                (Rcx, U64) => cursor.write_all(&[0x48, 0x8b, 0x48, offset]),

                //    mov    r8b,BYTE PTR [rax+$OFFSET]
                (R8, U8) => cursor.write_all(&[0x44, 0x8a, 0x40, offset]),
                //    mov    r8w,WORD PTR [rax+$OFFSET]
                (R8, U16) => cursor.write_all(&[0x66, 0x44, 0x8b, 0x40, offset]),
                //    mov    r8d,DWORD PTR [rax+$OFFSET]
                (R8, U32) => cursor.write_all(&[0x44, 0x8b, 0x40, offset]),
                //    mov    r8,QWORD PTR [rax+$OFFSET]
                (R8, U64) => cursor.write_all(&[0x4c, 0x8b, 0x40, offset]),

                //    mov    r9b,BYTE PTR [rax+$OFFSET]
                (R9, U8) => cursor.write_all(&[0x44, 0x8a, 0x48, offset]),
                //    mov    r9w,WORD PTR [rax+$OFFSET]
                (R9, U16) => cursor.write_all(&[0x66, 0x44, 0x8b, 0x48, offset]),
                //    mov    r9d,DWORD PTR [rax+$OFFSET]
                (R9, U32) => cursor.write_all(&[0x44, 0x8b, 0x48, offset]),
                //    mov    r9,QWORD PTR [rax+$OFFSET]
                (R9, U64) => cursor.write_all(&[0x4c, 0x8b, 0x48, offset]),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::{vmref_alloc_object, vmref_from_raw};
    use crate::jni::sys::jobject;
    use crate::thread::JvmThreadState;
    use crate::types::{DataValue, PrimitiveDataType};
    use cafebabe::mutf8::mstr;
    use std::mem::ManuallyDrop;

    static FAKE_JNIENV: () = ();
    static FAKE_JCLASS: () = ();

    extern "C" fn call_me(
        jnienv: *const (),
        jclass: *const (),
        a: i64,
        b: i16,
        c: i32,
        d: i8,
    ) -> u32 {
        assert!(std::ptr::eq(jnienv, &FAKE_JNIENV));
        assert!(std::ptr::eq(jclass, &FAKE_JCLASS));
        assert_eq!(a, 0x11223344_aabbccdd);
        assert_eq!(b, -12345);
        assert_eq!(c, 7);
        assert_eq!(d, 108);
        123
    }

    #[test]
    fn native_thunk_int_params() {
        let mut thunks = NativeThunks::default();
        let thunk = thunks.allocate().expect("alloc thunk");
        emit_thunk(
            &[
                DataType::Primitive(PrimitiveDataType::Long),
                DataType::Primitive(PrimitiveDataType::Short),
                DataType::Primitive(PrimitiveDataType::Int),
                DataType::Primitive(PrimitiveDataType::Byte),
            ],
            thunk.as_slice(),
            call_me as usize,
        )
        .expect("emit thunk");

        let mut args = [
            DataValue::Long(0x11223344_aabbccdd),
            DataValue::Short(-12345),
            DataValue::Int(7),
            DataValue::Byte(108),
        ];
        let args = FunctionArgs::from(&mut args[..]);

        let ret = thunk.invoke_inner(
            &FAKE_JNIENV as *const _ as *const _,
            &FAKE_JCLASS as *const _ as *const _,
            args,
        );
        assert_eq!(ret, 123);
    }
}
