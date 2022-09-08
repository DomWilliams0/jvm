use crate::jni::sys;
use crate::jni::sys::{jfieldID, jmethodID, JNIInvokeInterface_};

use crate::alloc::{vmref_from_raw, vmref_into_raw, VmRef};
use crate::class::Method;
use crate::storage::FieldId;
use std::ptr;

/// safety: *mut pointers that are non-Sync are reserved and never used
unsafe impl Sync for sys::JNIInvokeInterface_ {}
unsafe impl Sync for sys::JNINativeInterface_ {}

#[repr(transparent)]
struct UnsafeSyncPtr<T>(*const T);
unsafe impl Sync for UnsafeSyncPtr<sys::JNIInvokeInterface_> {}
unsafe impl Sync for UnsafeSyncPtr<sys::JNINativeInterface_> {}

#[repr(C)]
struct JniFieldId(FieldId);

#[repr(C)]
struct JniMethodId(VmRef<Method>);

pub fn current_javavm() -> *const sys::JavaVM {
    static JAVA_VM: sys::JNIInvokeInterface_ = JNIInvokeInterface_ {
        reserved0: ptr::null_mut(),
        reserved1: ptr::null_mut(),
        reserved2: ptr::null_mut(),
        DestroyJavaVM: Some(javavm::DestroyJavaVM),
        AttachCurrentThread: Some(javavm::AttachCurrentThread),
        DetachCurrentThread: Some(javavm::DetachCurrentThread),
        GetEnv: Some(javavm::GetEnv),
        AttachCurrentThreadAsDaemon: Some(javavm::AttachCurrentThreadAsDaemon),
    };
    static JAVA_VM_PTR: UnsafeSyncPtr<sys::JNIInvokeInterface_> =
        UnsafeSyncPtr(&JAVA_VM as sys::JavaVM);

    &JAVA_VM_PTR.0 as *const sys::JavaVM
}

pub fn global_env() -> *const sys::JNIEnv {
    static JNI_ENV: sys::JNINativeInterface_ = sys::JNINativeInterface_ {
        reserved0: ptr::null_mut(),
        reserved1: ptr::null_mut(),
        reserved2: ptr::null_mut(),
        reserved3: ptr::null_mut(),
        GetVersion: Some(jnienv::GetVersion),
        DefineClass: Some(jnienv::DefineClass),
        FindClass: Some(jnienv::FindClass),
        FromReflectedMethod: Some(jnienv::FromReflectedMethod),
        FromReflectedField: Some(jnienv::FromReflectedField),
        ToReflectedMethod: Some(jnienv::ToReflectedMethod),
        GetSuperclass: Some(jnienv::GetSuperclass),
        IsAssignableFrom: Some(jnienv::IsAssignableFrom),
        ToReflectedField: Some(jnienv::ToReflectedField),
        Throw: Some(jnienv::Throw),
        ThrowNew: Some(jnienv::ThrowNew),
        ExceptionOccurred: Some(jnienv::ExceptionOccurred),
        ExceptionDescribe: Some(jnienv::ExceptionDescribe),
        ExceptionClear: Some(jnienv::ExceptionClear),
        FatalError: Some(jnienv::FatalError),
        PushLocalFrame: Some(jnienv::PushLocalFrame),
        PopLocalFrame: Some(jnienv::PopLocalFrame),
        NewGlobalRef: Some(jnienv::NewGlobalRef),
        DeleteGlobalRef: Some(jnienv::DeleteGlobalRef),
        DeleteLocalRef: Some(jnienv::DeleteLocalRef),
        IsSameObject: Some(jnienv::IsSameObject),
        NewLocalRef: Some(jnienv::NewLocalRef),
        EnsureLocalCapacity: Some(jnienv::EnsureLocalCapacity),
        AllocObject: Some(jnienv::AllocObject),
        NewObject: Some(jnienv::NewObject),
        NewObjectV: Some(jnienv::NewObjectV),
        NewObjectA: Some(jnienv::NewObjectA),
        GetObjectClass: Some(jnienv::GetObjectClass),
        IsInstanceOf: Some(jnienv::IsInstanceOf),
        GetMethodID: Some(jnienv::GetMethodID),
        CallObjectMethod: Some(jnienv::CallObjectMethod),
        CallObjectMethodV: Some(jnienv::CallObjectMethodV),
        CallObjectMethodA: Some(jnienv::CallObjectMethodA),
        CallBooleanMethod: Some(jnienv::CallBooleanMethod),
        CallBooleanMethodV: Some(jnienv::CallBooleanMethodV),
        CallBooleanMethodA: Some(jnienv::CallBooleanMethodA),
        CallByteMethod: Some(jnienv::CallByteMethod),
        CallByteMethodV: Some(jnienv::CallByteMethodV),
        CallByteMethodA: Some(jnienv::CallByteMethodA),
        CallCharMethod: Some(jnienv::CallCharMethod),
        CallCharMethodV: Some(jnienv::CallCharMethodV),
        CallCharMethodA: Some(jnienv::CallCharMethodA),
        CallShortMethod: Some(jnienv::CallShortMethod),
        CallShortMethodV: Some(jnienv::CallShortMethodV),
        CallShortMethodA: Some(jnienv::CallShortMethodA),
        CallIntMethod: Some(jnienv::CallIntMethod),
        CallIntMethodV: Some(jnienv::CallIntMethodV),
        CallIntMethodA: Some(jnienv::CallIntMethodA),
        CallLongMethod: Some(jnienv::CallLongMethod),
        CallLongMethodV: Some(jnienv::CallLongMethodV),
        CallLongMethodA: Some(jnienv::CallLongMethodA),
        CallFloatMethod: Some(jnienv::CallFloatMethod),
        CallFloatMethodV: Some(jnienv::CallFloatMethodV),
        CallFloatMethodA: Some(jnienv::CallFloatMethodA),
        CallDoubleMethod: Some(jnienv::CallDoubleMethod),
        CallDoubleMethodV: Some(jnienv::CallDoubleMethodV),
        CallDoubleMethodA: Some(jnienv::CallDoubleMethodA),
        CallVoidMethod: Some(jnienv::CallVoidMethod),
        CallVoidMethodV: Some(jnienv::CallVoidMethodV),
        CallVoidMethodA: Some(jnienv::CallVoidMethodA),
        CallNonvirtualObjectMethod: Some(jnienv::CallNonvirtualObjectMethod),
        CallNonvirtualObjectMethodV: Some(jnienv::CallNonvirtualObjectMethodV),
        CallNonvirtualObjectMethodA: Some(jnienv::CallNonvirtualObjectMethodA),
        CallNonvirtualBooleanMethod: Some(jnienv::CallNonvirtualBooleanMethod),
        CallNonvirtualBooleanMethodV: Some(jnienv::CallNonvirtualBooleanMethodV),
        CallNonvirtualBooleanMethodA: Some(jnienv::CallNonvirtualBooleanMethodA),
        CallNonvirtualByteMethod: Some(jnienv::CallNonvirtualByteMethod),
        CallNonvirtualByteMethodV: Some(jnienv::CallNonvirtualByteMethodV),
        CallNonvirtualByteMethodA: Some(jnienv::CallNonvirtualByteMethodA),
        CallNonvirtualCharMethod: Some(jnienv::CallNonvirtualCharMethod),
        CallNonvirtualCharMethodV: Some(jnienv::CallNonvirtualCharMethodV),
        CallNonvirtualCharMethodA: Some(jnienv::CallNonvirtualCharMethodA),
        CallNonvirtualShortMethod: Some(jnienv::CallNonvirtualShortMethod),
        CallNonvirtualShortMethodV: Some(jnienv::CallNonvirtualShortMethodV),
        CallNonvirtualShortMethodA: Some(jnienv::CallNonvirtualShortMethodA),
        CallNonvirtualIntMethod: Some(jnienv::CallNonvirtualIntMethod),
        CallNonvirtualIntMethodV: Some(jnienv::CallNonvirtualIntMethodV),
        CallNonvirtualIntMethodA: Some(jnienv::CallNonvirtualIntMethodA),
        CallNonvirtualLongMethod: Some(jnienv::CallNonvirtualLongMethod),
        CallNonvirtualLongMethodV: Some(jnienv::CallNonvirtualLongMethodV),
        CallNonvirtualLongMethodA: Some(jnienv::CallNonvirtualLongMethodA),
        CallNonvirtualFloatMethod: Some(jnienv::CallNonvirtualFloatMethod),
        CallNonvirtualFloatMethodV: Some(jnienv::CallNonvirtualFloatMethodV),
        CallNonvirtualFloatMethodA: Some(jnienv::CallNonvirtualFloatMethodA),
        CallNonvirtualDoubleMethod: Some(jnienv::CallNonvirtualDoubleMethod),
        CallNonvirtualDoubleMethodV: Some(jnienv::CallNonvirtualDoubleMethodV),
        CallNonvirtualDoubleMethodA: Some(jnienv::CallNonvirtualDoubleMethodA),
        CallNonvirtualVoidMethod: Some(jnienv::CallNonvirtualVoidMethod),
        CallNonvirtualVoidMethodV: Some(jnienv::CallNonvirtualVoidMethodV),
        CallNonvirtualVoidMethodA: Some(jnienv::CallNonvirtualVoidMethodA),
        GetFieldID: Some(jnienv::GetFieldID),
        GetObjectField: Some(jnienv::GetObjectField),
        GetBooleanField: Some(jnienv::GetBooleanField),
        GetByteField: Some(jnienv::GetByteField),
        GetCharField: Some(jnienv::GetCharField),
        GetShortField: Some(jnienv::GetShortField),
        GetIntField: Some(jnienv::GetIntField),
        GetLongField: Some(jnienv::GetLongField),
        GetFloatField: Some(jnienv::GetFloatField),
        GetDoubleField: Some(jnienv::GetDoubleField),
        SetObjectField: Some(jnienv::SetObjectField),
        SetBooleanField: Some(jnienv::SetBooleanField),
        SetByteField: Some(jnienv::SetByteField),
        SetCharField: Some(jnienv::SetCharField),
        SetShortField: Some(jnienv::SetShortField),
        SetIntField: Some(jnienv::SetIntField),
        SetLongField: Some(jnienv::SetLongField),
        SetFloatField: Some(jnienv::SetFloatField),
        SetDoubleField: Some(jnienv::SetDoubleField),
        GetStaticMethodID: Some(jnienv::GetStaticMethodID),
        CallStaticObjectMethod: Some(jnienv::CallStaticObjectMethod),
        CallStaticObjectMethodV: Some(jnienv::CallStaticObjectMethodV),
        CallStaticObjectMethodA: Some(jnienv::CallStaticObjectMethodA),
        CallStaticBooleanMethod: Some(jnienv::CallStaticBooleanMethod),
        CallStaticBooleanMethodV: Some(jnienv::CallStaticBooleanMethodV),
        CallStaticBooleanMethodA: Some(jnienv::CallStaticBooleanMethodA),
        CallStaticByteMethod: Some(jnienv::CallStaticByteMethod),
        CallStaticByteMethodV: Some(jnienv::CallStaticByteMethodV),
        CallStaticByteMethodA: Some(jnienv::CallStaticByteMethodA),
        CallStaticCharMethod: Some(jnienv::CallStaticCharMethod),
        CallStaticCharMethodV: Some(jnienv::CallStaticCharMethodV),
        CallStaticCharMethodA: Some(jnienv::CallStaticCharMethodA),
        CallStaticShortMethod: Some(jnienv::CallStaticShortMethod),
        CallStaticShortMethodV: Some(jnienv::CallStaticShortMethodV),
        CallStaticShortMethodA: Some(jnienv::CallStaticShortMethodA),
        CallStaticIntMethod: Some(jnienv::CallStaticIntMethod),
        CallStaticIntMethodV: Some(jnienv::CallStaticIntMethodV),
        CallStaticIntMethodA: Some(jnienv::CallStaticIntMethodA),
        CallStaticLongMethod: Some(jnienv::CallStaticLongMethod),
        CallStaticLongMethodV: Some(jnienv::CallStaticLongMethodV),
        CallStaticLongMethodA: Some(jnienv::CallStaticLongMethodA),
        CallStaticFloatMethod: Some(jnienv::CallStaticFloatMethod),
        CallStaticFloatMethodV: Some(jnienv::CallStaticFloatMethodV),
        CallStaticFloatMethodA: Some(jnienv::CallStaticFloatMethodA),
        CallStaticDoubleMethod: Some(jnienv::CallStaticDoubleMethod),
        CallStaticDoubleMethodV: Some(jnienv::CallStaticDoubleMethodV),
        CallStaticDoubleMethodA: Some(jnienv::CallStaticDoubleMethodA),
        CallStaticVoidMethod: Some(jnienv::CallStaticVoidMethod),
        CallStaticVoidMethodV: Some(jnienv::CallStaticVoidMethodV),
        CallStaticVoidMethodA: Some(jnienv::CallStaticVoidMethodA),
        GetStaticFieldID: Some(jnienv::GetStaticFieldID),
        GetStaticObjectField: Some(jnienv::GetStaticObjectField),
        GetStaticBooleanField: Some(jnienv::GetStaticBooleanField),
        GetStaticByteField: Some(jnienv::GetStaticByteField),
        GetStaticCharField: Some(jnienv::GetStaticCharField),
        GetStaticShortField: Some(jnienv::GetStaticShortField),
        GetStaticIntField: Some(jnienv::GetStaticIntField),
        GetStaticLongField: Some(jnienv::GetStaticLongField),
        GetStaticFloatField: Some(jnienv::GetStaticFloatField),
        GetStaticDoubleField: Some(jnienv::GetStaticDoubleField),
        SetStaticObjectField: Some(jnienv::SetStaticObjectField),
        SetStaticBooleanField: Some(jnienv::SetStaticBooleanField),
        SetStaticByteField: Some(jnienv::SetStaticByteField),
        SetStaticCharField: Some(jnienv::SetStaticCharField),
        SetStaticShortField: Some(jnienv::SetStaticShortField),
        SetStaticIntField: Some(jnienv::SetStaticIntField),
        SetStaticLongField: Some(jnienv::SetStaticLongField),
        SetStaticFloatField: Some(jnienv::SetStaticFloatField),
        SetStaticDoubleField: Some(jnienv::SetStaticDoubleField),
        NewString: Some(jnienv::NewString),
        GetStringLength: Some(jnienv::GetStringLength),
        GetStringChars: Some(jnienv::GetStringChars),
        ReleaseStringChars: Some(jnienv::ReleaseStringChars),
        NewStringUTF: Some(jnienv::NewStringUTF),
        GetStringUTFLength: Some(jnienv::GetStringUTFLength),
        GetStringUTFChars: Some(jnienv::GetStringUTFChars),
        ReleaseStringUTFChars: Some(jnienv::ReleaseStringUTFChars),
        GetArrayLength: Some(jnienv::GetArrayLength),
        NewObjectArray: Some(jnienv::NewObjectArray),
        GetObjectArrayElement: Some(jnienv::GetObjectArrayElement),
        SetObjectArrayElement: Some(jnienv::SetObjectArrayElement),
        NewBooleanArray: Some(jnienv::NewBooleanArray),
        NewByteArray: Some(jnienv::NewByteArray),
        NewCharArray: Some(jnienv::NewCharArray),
        NewShortArray: Some(jnienv::NewShortArray),
        NewIntArray: Some(jnienv::NewIntArray),
        NewLongArray: Some(jnienv::NewLongArray),
        NewFloatArray: Some(jnienv::NewFloatArray),
        NewDoubleArray: Some(jnienv::NewDoubleArray),
        GetBooleanArrayElements: Some(jnienv::GetBooleanArrayElements),
        GetByteArrayElements: Some(jnienv::GetByteArrayElements),
        GetCharArrayElements: Some(jnienv::GetCharArrayElements),
        GetShortArrayElements: Some(jnienv::GetShortArrayElements),
        GetIntArrayElements: Some(jnienv::GetIntArrayElements),
        GetLongArrayElements: Some(jnienv::GetLongArrayElements),
        GetFloatArrayElements: Some(jnienv::GetFloatArrayElements),
        GetDoubleArrayElements: Some(jnienv::GetDoubleArrayElements),
        ReleaseBooleanArrayElements: Some(jnienv::ReleaseBooleanArrayElements),
        ReleaseByteArrayElements: Some(jnienv::ReleaseByteArrayElements),
        ReleaseCharArrayElements: Some(jnienv::ReleaseCharArrayElements),
        ReleaseShortArrayElements: Some(jnienv::ReleaseShortArrayElements),
        ReleaseIntArrayElements: Some(jnienv::ReleaseIntArrayElements),
        ReleaseLongArrayElements: Some(jnienv::ReleaseLongArrayElements),
        ReleaseFloatArrayElements: Some(jnienv::ReleaseFloatArrayElements),
        ReleaseDoubleArrayElements: Some(jnienv::ReleaseDoubleArrayElements),
        GetBooleanArrayRegion: Some(jnienv::GetBooleanArrayRegion),
        GetByteArrayRegion: Some(jnienv::GetByteArrayRegion),
        GetCharArrayRegion: Some(jnienv::GetCharArrayRegion),
        GetShortArrayRegion: Some(jnienv::GetShortArrayRegion),
        GetIntArrayRegion: Some(jnienv::GetIntArrayRegion),
        GetLongArrayRegion: Some(jnienv::GetLongArrayRegion),
        GetFloatArrayRegion: Some(jnienv::GetFloatArrayRegion),
        GetDoubleArrayRegion: Some(jnienv::GetDoubleArrayRegion),
        SetBooleanArrayRegion: Some(jnienv::SetBooleanArrayRegion),
        SetByteArrayRegion: Some(jnienv::SetByteArrayRegion),
        SetCharArrayRegion: Some(jnienv::SetCharArrayRegion),
        SetShortArrayRegion: Some(jnienv::SetShortArrayRegion),
        SetIntArrayRegion: Some(jnienv::SetIntArrayRegion),
        SetLongArrayRegion: Some(jnienv::SetLongArrayRegion),
        SetFloatArrayRegion: Some(jnienv::SetFloatArrayRegion),
        SetDoubleArrayRegion: Some(jnienv::SetDoubleArrayRegion),
        RegisterNatives: Some(jnienv::RegisterNatives),
        UnregisterNatives: Some(jnienv::UnregisterNatives),
        MonitorEnter: Some(jnienv::MonitorEnter),
        MonitorExit: Some(jnienv::MonitorExit),
        GetJavaVM: Some(jnienv::GetJavaVM),
        GetStringRegion: Some(jnienv::GetStringRegion),
        GetStringUTFRegion: Some(jnienv::GetStringUTFRegion),
        GetPrimitiveArrayCritical: Some(jnienv::GetPrimitiveArrayCritical),
        ReleasePrimitiveArrayCritical: Some(jnienv::ReleasePrimitiveArrayCritical),
        GetStringCritical: Some(jnienv::GetStringCritical),
        ReleaseStringCritical: Some(jnienv::ReleaseStringCritical),
        NewWeakGlobalRef: Some(jnienv::NewWeakGlobalRef),
        DeleteWeakGlobalRef: Some(jnienv::DeleteWeakGlobalRef),
        ExceptionCheck: Some(jnienv::ExceptionCheck),
        NewDirectByteBuffer: Some(jnienv::NewDirectByteBuffer),
        GetDirectBufferAddress: Some(jnienv::GetDirectBufferAddress),
        GetDirectBufferCapacity: Some(jnienv::GetDirectBufferCapacity),
        GetObjectRefType: Some(jnienv::GetObjectRefType),
    };

    static JNI_ENV_PTR: UnsafeSyncPtr<sys::JNINativeInterface_> =
        UnsafeSyncPtr(&JNI_ENV as sys::JNIEnv);

    &JNI_ENV_PTR.0 as *const sys::JNIEnv
}

mod javavm {
    #![allow(non_snake_case, unused_variables)]
    use crate::jni::api::global_env;
    use crate::jni::sys::*;
    use crate::jni::JNI_VERSION;
    use log::trace;
    use std::ptr;

    pub extern "C" fn DestroyJavaVM(arg1: *mut JavaVM) -> jint {
        trace!("javavm::DestroyJavaVM()");
        todo!("DestroyJavaVM")
    }

    pub extern "C" fn AttachCurrentThread(
        arg1: *mut JavaVM,
        arg2: *mut *mut ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> jint {
        trace!("javavm::AttachCurrentThread({:?}, {:?})", arg2, arg3);
        todo!("AttachCurrentThread")
    }

    pub extern "C" fn DetachCurrentThread(arg1: *mut JavaVM) -> jint {
        trace!("javavm::DetachCurrentThread()");
        todo!("DetachCurrentThread")
    }

    pub extern "C" fn GetEnv(
        vm: *mut JavaVM,
        env_out: *mut *mut ::std::os::raw::c_void,
        version: jint,
    ) -> jint {
        trace!("javavm::GetEnv({:?}, {:?})", env_out, version);
        // TODO get actual env for current thread, rather than a global

        // fake thread-local env by only returning a global to an already initialised jvm thread
        let attached = crate::thread::is_initialised();

        let (env, ret) = if !attached {
            (None, JNI_EDETACHED)
        } else if version > JNI_VERSION as i32 {
            (None, JNI_EVERSION)
        } else {
            (Some(global_env()), JNI_OK as i32)
        };

        unsafe {
            *env_out = env.map(|ptr| ptr as *mut _).unwrap_or_else(ptr::null_mut);
        }
        ret
    }

    pub extern "C" fn AttachCurrentThreadAsDaemon(
        arg1: *mut JavaVM,
        arg2: *mut *mut ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> jint {
        trace!(
            "javavm::AttachCurrentThreadAsDaemon({:?}, {:?})",
            arg2,
            arg3
        );
        todo!("AttachCurrentThreadAsDaemon")
    }
}

mod jnienv {
    #![allow(non_snake_case, unused_variables)]
    use crate::alloc::{
        vmref_as_raw, vmref_from_raw, vmref_increment, vmref_into_raw, vmref_is_null, vmref_ptr,
        VmRef,
    };
    use crate::class::{Class, Object, WhichLoader};

    use crate::error::{Throwables, VmResult};
    use crate::exec_helper::ArrayType;
    use crate::jni::api::{JniFieldId, JniMethodId};
    use crate::jni::sys::*;
    use crate::jni::JNI_VERSION;
    use crate::thread;
    use crate::types::{DataType, DataValue};
    use cafebabe::mutf8::mstr;
    use cafebabe::MethodAccessFlags;
    use itertools::repeat_n;
    use log::*;
    use std::ffi::{CStr, CString};
    use std::mem::ManuallyDrop;
    use std::num::TryFromIntError;
    use std::ptr;
    use std::ptr::{null, null_mut};
    use std::sync::Arc;

    unsafe fn as_string<'a>(s: *const ::std::os::raw::c_char) -> &'a mstr {
        let cstr = CStr::from_ptr(s);
        mstr::from_mutf8(cstr.to_bytes())
    }

    unsafe fn as_vmref<T>(obj: *const ::std::os::raw::c_void) -> ManuallyDrop<VmRef<T>> {
        ManuallyDrop::new(vmref_from_raw(obj as *const T))
    }

    /// If true, NPE exception has been raised
    fn is_null_throwing<T>(ptr: *const T) -> bool {
        if ptr.is_null() {
            let t = thread::get();
            t.set_exception(Throwables::NullPointerException.into());
            true
        } else {
            false
        }
    }

    fn jnull<T>() -> *mut T {
        null_mut()
    }

    pub extern "C" fn GetVersion(env: *mut JNIEnv) -> jint {
        trace!("jni::GetVersion()");
        JNI_VERSION as jint
    }

    pub extern "C" fn DefineClass(
        env: *mut JNIEnv,
        arg2: *const ::std::os::raw::c_char,
        arg3: jobject,
        arg4: *const jbyte,
        arg5: jsize,
    ) -> jclass {
        trace!(
            "jni::DefineClass({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("DefineClass")
    }

    pub extern "C" fn FindClass(env: *mut JNIEnv, name: *const ::std::os::raw::c_char) -> jclass {
        trace!("jni::FindClass({:?})", name);
        let name = unsafe { as_string(name) };

        debug!("FindClass({})", name);

        let thread = thread::get();

        // choose classloader based on calling class
        let class_loader = thread
            .interpreter()
            .with_frame(1, |frame| {
                let loader = frame
                    .class_and_method()
                    .class()
                    .map(|cls| cls.loader().clone());

                trace!("using loader {:?} from calling class {:?}", loader, frame);
                loader
            })
            .flatten()
            .unwrap_or(WhichLoader::Bootstrap);

        // dont hold interpreter ref while loading class

        let interp = thread.interpreter();
        match thread
            .global()
            .class_loader()
            .load_class(name, class_loader)
        {
            Ok(cls) => {
                // add local ref
                interp.with_current_native_frame(|frame| frame.add_local_ref(&cls));

                vmref_into_raw(cls) as jclass
            }
            Err(err) => {
                // TODO set exception
                warn!("FindClass failed: {:?}", err);
                todo!("set jni exception for {:?}", err);
                ptr::null_mut()
            }
        }
    }

    pub extern "C" fn FromReflectedMethod(env: *mut JNIEnv, arg2: jobject) -> jmethodID {
        trace!("jni::FromReflectedMethod({:?})", arg2);
        todo!("FromReflectedMethod")
    }

    pub extern "C" fn FromReflectedField(env: *mut JNIEnv, arg2: jobject) -> jfieldID {
        trace!("jni::FromReflectedField({:?})", arg2);
        todo!("FromReflectedField")
    }

    pub extern "C" fn ToReflectedMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: jboolean,
    ) -> jobject {
        trace!("jni::ToReflectedMethod({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("ToReflectedMethod")
    }

    pub extern "C" fn GetSuperclass(env: *mut JNIEnv, arg2: jclass) -> jclass {
        trace!("jni::GetSuperclass({:?})", arg2);
        todo!("GetSuperclass")
    }

    pub extern "C" fn IsAssignableFrom(env: *mut JNIEnv, arg2: jclass, arg3: jclass) -> jboolean {
        trace!("jni::IsAssignableFrom({:?}, {:?})", arg2, arg3);
        todo!("IsAssignableFrom")
    }

    pub extern "C" fn ToReflectedField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jboolean,
    ) -> jobject {
        trace!("jni::ToReflectedField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("ToReflectedField")
    }

    pub extern "C" fn Throw(env: *mut JNIEnv, arg2: jthrowable) -> jint {
        trace!("jni::Throw({:?})", arg2);
        todo!("Throw")
    }

    pub extern "C" fn ThrowNew(
        env: *mut JNIEnv,
        cls: jclass,
        msg: *const ::std::os::raw::c_char,
    ) -> jint {
        trace!("jni::ThrowNew({:?}, {:?})", cls, msg);

        if cls.is_null() || msg.is_null() {
            return -1;
        }

        let (class, msg) = unsafe { (as_vmref::<Class>(cls), CStr::from_ptr(msg)) };

        todo!("ThrowNew({:?}, {:?})", class.name(), msg)
    }

    pub extern "C" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
        trace!("jni::ExceptionOccurred()");
        let t = thread::get();
        match t.exception() {
            None => jnull(),
            Some(e) => vmref_into_raw(e) as jthrowable,
        }
    }

    pub extern "C" fn ExceptionDescribe(env: *mut JNIEnv) {
        trace!("jni::ExceptionDescribe()");
        todo!("ExceptionDescribe")
    }

    pub extern "C" fn ExceptionClear(env: *mut JNIEnv) {
        trace!("jni::ExceptionClear()");
        let t = thread::get();
        t.exception().take();
    }

    pub extern "C" fn FatalError(env: *mut JNIEnv, arg2: *const ::std::os::raw::c_char) {
        trace!("jni::FatalError({:?})", arg2);
        todo!("FatalError")
    }

    pub extern "C" fn PushLocalFrame(env: *mut JNIEnv, arg2: jint) -> jint {
        trace!("jni::PushLocalFrame({:?})", arg2);
        todo!("PushLocalFrame")
    }

    pub extern "C" fn PopLocalFrame(env: *mut JNIEnv, arg2: jobject) -> jobject {
        trace!("jni::PopLocalFrame({:?})", arg2);
        todo!("PopLocalFrame")
    }

    pub extern "C" fn NewGlobalRef(env: *mut JNIEnv, obj: jobject) -> jobject {
        // TODO keep track of global references in jvm or is it ok to leak them like this?
        trace!("jni::NewGlobalRef({:?})", obj);

        // obj must have come from us, and is already a full reference, so ensure we dont drop it
        let vmobj = unsafe { as_vmref::<()>(obj) };

        // bump ref count
        let refs = vmref_increment(&vmobj);

        debug!(
            "incremented ref count of object at {:?} to {}",
            vmref_ptr(&vmobj),
            refs
        );

        // return the same object
        obj
    }

    pub extern "C" fn DeleteGlobalRef(env: *mut JNIEnv, arg2: jobject) {
        trace!("jni::DeleteGlobalRef({:?})", arg2);
        todo!("DeleteGlobalRef")
    }

    pub extern "C" fn DeleteLocalRef(env: *mut JNIEnv, arg2: jobject) {
        trace!("jni::DeleteLocalRef({:?})", arg2);
        // TODO actually do something
    }

    pub extern "C" fn IsSameObject(env: *mut JNIEnv, arg2: jobject, arg3: jobject) -> jboolean {
        trace!("jni::IsSameObject({:?}, {:?})", arg2, arg3);
        todo!("IsSameObject")
    }

    pub extern "C" fn NewLocalRef(env: *mut JNIEnv, obj: jobject) -> jobject {
        trace!("jni::NewLocalRef({:?})", obj);
        let obj_ref = unsafe { as_vmref::<Object>(obj) };
        if vmref_is_null(&obj_ref) {
            return obj;
        }

        let t = thread::get();
        let interpreter = t.interpreter();
        let mut state = interpreter.state_mut();
        let frame = state.current_native_frame_mut();
        frame.add_local_ref(&obj_ref);

        // return same ref
        obj
    }

    pub extern "C" fn EnsureLocalCapacity(env: *mut JNIEnv, arg2: jint) -> jint {
        trace!("jni::EnsureLocalCapacity({:?})", arg2);
        todo!("EnsureLocalCapacity")
    }

    pub extern "C" fn AllocObject(env: *mut JNIEnv, arg2: jclass) -> jobject {
        trace!("jni::AllocObject({:?})", arg2);
        todo!("AllocObject")
    }

    pub unsafe extern "C" fn NewObject(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jobject {
        trace!("jni::NewObject({:?}, {:?}, ...)", arg2, arg3);
        todo!("NewObject")
    }

    pub extern "C" fn NewObjectV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jobject {
        trace!("jni::NewObjectV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("NewObjectV")
    }

    pub extern "C" fn NewObjectA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jobject {
        trace!("jni::NewObjectA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("NewObjectA")
    }

    pub extern "C" fn GetObjectClass(env: *mut JNIEnv, arg2: jobject) -> jclass {
        trace!("jni::GetObjectClass({:?})", arg2);
        todo!("GetObjectClass")
    }

    pub extern "C" fn IsInstanceOf(env: *mut JNIEnv, arg2: jobject, arg3: jclass) -> jboolean {
        trace!("jni::IsInstanceOf({:?}, {:?})", arg2, arg3);
        todo!("IsInstanceOf")
    }

    pub extern "C" fn GetMethodID(
        env: *mut JNIEnv,
        class: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> jmethodID {
        trace!("jni::GetMethodID({:?}, {:?}, {:?})", class, name, sig);
        let class = unsafe { as_vmref::<Class>(class) };
        let name = unsafe { as_string(name) };
        let sig = unsafe { as_string(sig) };

        // TODO throw exception instead of panic
        let method = class
            .find_method_recursive_in_superclasses(
                name,
                sig,
                MethodAccessFlags::empty(),
                MethodAccessFlags::empty(),
            )
            .unwrap_or_else(|| panic!("method {:?}::{:?} ({}) not found", class.name(), name, sig));

        trace!(
            "GetMethodId({:?}::{} (type {:?})) -> {}",
            class.name(),
            name,
            sig,
            method,
        );

        JniMethodId(method).into()
    }

    pub unsafe extern "C" fn CallObjectMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jobject {
        trace!("jni::CallObjectMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallObjectMethod")
    }

    pub extern "C" fn CallObjectMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jobject {
        trace!("jni::CallObjectMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallObjectMethodV")
    }

    pub extern "C" fn CallObjectMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jobject {
        trace!("jni::CallObjectMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallObjectMethodA")
    }

    pub unsafe extern "C" fn CallBooleanMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jboolean {
        trace!("jni::CallBooleanMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallBooleanMethod")
    }

    pub extern "C" fn CallBooleanMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jboolean {
        trace!(
            "jni::CallBooleanMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallBooleanMethodV")
    }

    pub extern "C" fn CallBooleanMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jboolean {
        trace!(
            "jni::CallBooleanMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallBooleanMethodA")
    }

    pub unsafe extern "C" fn CallByteMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jbyte {
        trace!("jni::CallByteMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallByteMethod")
    }

    pub extern "C" fn CallByteMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jbyte {
        trace!("jni::CallByteMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallByteMethodV")
    }

    pub extern "C" fn CallByteMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jbyte {
        trace!("jni::CallByteMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallByteMethodA")
    }

    pub unsafe extern "C" fn CallCharMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jchar {
        trace!("jni::CallCharMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallCharMethod")
    }

    pub extern "C" fn CallCharMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jchar {
        trace!("jni::CallCharMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallCharMethodV")
    }

    pub extern "C" fn CallCharMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jchar {
        trace!("jni::CallCharMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallCharMethodA")
    }

    pub unsafe extern "C" fn CallShortMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jshort {
        trace!("jni::CallShortMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallShortMethod")
    }

    pub extern "C" fn CallShortMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jshort {
        trace!("jni::CallShortMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallShortMethodV")
    }

    pub extern "C" fn CallShortMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jshort {
        trace!("jni::CallShortMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallShortMethodA")
    }

    pub unsafe extern "C" fn CallIntMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jint {
        trace!("jni::CallIntMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallIntMethod")
    }

    pub extern "C" fn CallIntMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jint {
        trace!("jni::CallIntMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallIntMethodV")
    }

    pub extern "C" fn CallIntMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jint {
        trace!("jni::CallIntMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallIntMethodA")
    }

    pub unsafe extern "C" fn CallLongMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jlong {
        trace!("jni::CallLongMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallLongMethod")
    }

    pub extern "C" fn CallLongMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jlong {
        trace!("jni::CallLongMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallLongMethodV")
    }

    pub extern "C" fn CallLongMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jlong {
        trace!("jni::CallLongMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallLongMethodA")
    }

    pub unsafe extern "C" fn CallFloatMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jfloat {
        trace!("jni::CallFloatMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallFloatMethod")
    }

    pub extern "C" fn CallFloatMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jfloat {
        trace!("jni::CallFloatMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallFloatMethodV")
    }

    pub extern "C" fn CallFloatMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jfloat {
        trace!("jni::CallFloatMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallFloatMethodA")
    }

    pub unsafe extern "C" fn CallDoubleMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jdouble {
        trace!("jni::CallDoubleMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallDoubleMethod")
    }

    pub extern "C" fn CallDoubleMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jdouble {
        trace!("jni::CallDoubleMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallDoubleMethodV")
    }

    pub extern "C" fn CallDoubleMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jdouble {
        trace!("jni::CallDoubleMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallDoubleMethodA")
    }

    pub unsafe extern "C" fn CallVoidMethod(env: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) {
        trace!("jni::CallVoidMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallVoidMethod")
    }

    pub extern "C" fn CallVoidMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) {
        trace!("jni::CallVoidMethodV({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallVoidMethodV")
    }

    pub extern "C" fn CallVoidMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) {
        trace!("jni::CallVoidMethodA({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("CallVoidMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualObjectMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jobject {
        trace!(
            "jni::CallNonvirtualObjectMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualObjectMethod")
    }

    pub extern "C" fn CallNonvirtualObjectMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jobject {
        trace!(
            "jni::CallNonvirtualObjectMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualObjectMethodV")
    }

    pub extern "C" fn CallNonvirtualObjectMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jobject {
        trace!(
            "jni::CallNonvirtualObjectMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualObjectMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualBooleanMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jboolean {
        trace!(
            "jni::CallNonvirtualBooleanMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualBooleanMethod")
    }

    pub extern "C" fn CallNonvirtualBooleanMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jboolean {
        trace!(
            "jni::CallNonvirtualBooleanMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualBooleanMethodV")
    }

    pub extern "C" fn CallNonvirtualBooleanMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jboolean {
        trace!(
            "jni::CallNonvirtualBooleanMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualBooleanMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualByteMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jbyte {
        trace!(
            "jni::CallNonvirtualByteMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualByteMethod")
    }

    pub extern "C" fn CallNonvirtualByteMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jbyte {
        trace!(
            "jni::CallNonvirtualByteMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualByteMethodV")
    }

    pub extern "C" fn CallNonvirtualByteMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jbyte {
        trace!(
            "jni::CallNonvirtualByteMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualByteMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualCharMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jchar {
        trace!(
            "jni::CallNonvirtualCharMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualCharMethod")
    }

    pub extern "C" fn CallNonvirtualCharMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jchar {
        trace!(
            "jni::CallNonvirtualCharMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualCharMethodV")
    }

    pub extern "C" fn CallNonvirtualCharMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jchar {
        trace!(
            "jni::CallNonvirtualCharMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualCharMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualShortMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jshort {
        trace!(
            "jni::CallNonvirtualShortMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualShortMethod")
    }

    pub extern "C" fn CallNonvirtualShortMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jshort {
        trace!(
            "jni::CallNonvirtualShortMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualShortMethodV")
    }

    pub extern "C" fn CallNonvirtualShortMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jshort {
        trace!(
            "jni::CallNonvirtualShortMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualShortMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualIntMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jint {
        trace!(
            "jni::CallNonvirtualIntMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualIntMethod")
    }

    pub extern "C" fn CallNonvirtualIntMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jint {
        trace!(
            "jni::CallNonvirtualIntMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualIntMethodV")
    }

    pub extern "C" fn CallNonvirtualIntMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jint {
        trace!(
            "jni::CallNonvirtualIntMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualIntMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualLongMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jlong {
        trace!(
            "jni::CallNonvirtualLongMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualLongMethod")
    }

    pub extern "C" fn CallNonvirtualLongMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jlong {
        trace!(
            "jni::CallNonvirtualLongMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualLongMethodV")
    }

    pub extern "C" fn CallNonvirtualLongMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jlong {
        trace!(
            "jni::CallNonvirtualLongMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualLongMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualFloatMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jfloat {
        trace!(
            "jni::CallNonvirtualFloatMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualFloatMethod")
    }

    pub extern "C" fn CallNonvirtualFloatMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jfloat {
        trace!(
            "jni::CallNonvirtualFloatMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualFloatMethodV")
    }

    pub extern "C" fn CallNonvirtualFloatMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jfloat {
        trace!(
            "jni::CallNonvirtualFloatMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualFloatMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualDoubleMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jdouble {
        trace!(
            "jni::CallNonvirtualDoubleMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualDoubleMethod")
    }

    pub extern "C" fn CallNonvirtualDoubleMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jdouble {
        trace!(
            "jni::CallNonvirtualDoubleMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualDoubleMethodV")
    }

    pub extern "C" fn CallNonvirtualDoubleMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jdouble {
        trace!(
            "jni::CallNonvirtualDoubleMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualDoubleMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualVoidMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) {
        trace!(
            "jni::CallNonvirtualVoidMethod({:?}, {:?}, {:?}, ...)",
            arg2,
            arg3,
            arg4
        );
        todo!("CallNonvirtualVoidMethod")
    }

    pub extern "C" fn CallNonvirtualVoidMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) {
        trace!(
            "jni::CallNonvirtualVoidMethodV({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualVoidMethodV")
    }

    pub extern "C" fn CallNonvirtualVoidMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) {
        trace!(
            "jni::CallNonvirtualVoidMethodA({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("CallNonvirtualVoidMethodA")
    }

    pub extern "C" fn GetFieldID(
        env: *mut JNIEnv,
        class: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> jfieldID {
        trace!("jni::GetFieldID({:?}, {:?}, {:?})", class, name, sig);
        let class = unsafe { as_vmref::<Class>(class) };
        let name = unsafe { as_string(name) };
        let sig = unsafe { as_string(sig) };

        let desc = DataType::from_descriptor(sig).expect("bad signature");
        // TODO throw exception instead of panic
        let field = class
            .find_instance_field_recursive(name, &desc)
            .unwrap_or_else(|| panic!("field {:?}.{:?} ({}) not found", class.name(), name, sig));

        trace!(
            "GetFieldId({:?}.{} (type {:?})) -> {:?}",
            class.name(),
            name,
            sig,
            field
        );

        JniFieldId(field).into()
    }

    pub extern "C" fn GetObjectField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jobject {
        trace!("jni::GetObjectField({:?}, {:?})", arg2, arg3);
        todo!("GetObjectField")
    }

    pub extern "C" fn GetBooleanField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jboolean {
        trace!("jni::GetBooleanField({:?}, {:?})", arg2, arg3);
        todo!("GetBooleanField")
    }

    pub extern "C" fn GetByteField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jbyte {
        trace!("jni::GetByteField({:?}, {:?})", arg2, arg3);
        todo!("GetByteField")
    }

    pub extern "C" fn GetCharField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jchar {
        trace!("jni::GetCharField({:?}, {:?})", arg2, arg3);
        todo!("GetCharField")
    }

    pub extern "C" fn GetShortField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jshort {
        trace!("jni::GetShortField({:?}, {:?})", arg2, arg3);
        todo!("GetShortField")
    }

    pub extern "C" fn GetIntField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jint {
        trace!("jni::GetIntField({:?}, {:?})", arg2, arg3);
        todo!("GetIntField")
    }

    pub extern "C" fn GetLongField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jlong {
        trace!("jni::GetLongField({:?}, {:?})", arg2, arg3);
        todo!("GetLongField")
    }

    pub extern "C" fn GetFloatField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jfloat {
        trace!("jni::GetFloatField({:?}, {:?})", arg2, arg3);
        todo!("GetFloatField")
    }

    pub extern "C" fn GetDoubleField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jdouble {
        trace!("jni::GetDoubleField({:?}, {:?})", arg2, arg3);
        todo!("GetDoubleField")
    }

    pub extern "C" fn SetObjectField(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jfieldID,
        arg4: jobject,
    ) {
        trace!("jni::SetObjectField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetObjectField")
    }

    pub extern "C" fn SetBooleanField(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jfieldID,
        arg4: jboolean,
    ) {
        trace!("jni::SetBooleanField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetBooleanField")
    }

    pub extern "C" fn SetByteField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jbyte) {
        trace!("jni::SetByteField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetByteField")
    }

    pub extern "C" fn SetCharField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jchar) {
        trace!("jni::SetCharField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetCharField")
    }

    pub extern "C" fn SetShortField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jshort) {
        trace!("jni::SetShortField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetShortField")
    }

    pub extern "C" fn SetIntField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jint) {
        trace!("jni::SetIntField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetIntField")
    }

    pub extern "C" fn SetLongField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jlong) {
        trace!("jni::SetLongField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetLongField")
    }

    pub extern "C" fn SetFloatField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jfloat) {
        trace!("jni::SetFloatField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetFloatField")
    }

    pub extern "C" fn SetDoubleField(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jfieldID,
        arg4: jdouble,
    ) {
        trace!("jni::SetDoubleField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetDoubleField")
    }

    pub extern "C" fn GetStaticMethodID(
        env: *mut JNIEnv,
        class: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> jmethodID {
        trace!("jni::GetStaticMethodID({:?}, {:?}, {:?})", class, name, sig);
        let class = unsafe { as_vmref::<Class>(class) };
        let name = unsafe { as_string(name) };
        let sig = unsafe { as_string(sig) };

        // TODO throw exception instead of panic
        let method = class
            .find_method_recursive_in_superclasses(
                name,
                sig,
                MethodAccessFlags::STATIC,
                MethodAccessFlags::empty(),
            )
            .unwrap_or_else(|| {
                panic!(
                    "static method {:?}::{:?} ({}) not found",
                    class.name(),
                    name,
                    sig
                )
            });

        trace!(
            "GetStaticMethodId({:?}::{} (type {:?})) -> {}",
            class.name(),
            name,
            sig,
            method,
        );

        JniMethodId(method).into()
    }

    pub unsafe extern "C" fn CallStaticObjectMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jobject {
        trace!("jni::CallStaticObjectMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticObjectMethod")
    }

    pub extern "C" fn CallStaticObjectMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jobject {
        trace!(
            "jni::CallStaticObjectMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticObjectMethodV")
    }

    pub extern "C" fn CallStaticObjectMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jobject {
        trace!(
            "jni::CallStaticObjectMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticObjectMethodA")
    }

    pub unsafe extern "C" fn CallStaticBooleanMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jboolean {
        trace!("jni::CallStaticBooleanMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticBooleanMethod")
    }

    pub extern "C" fn CallStaticBooleanMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jboolean {
        trace!(
            "jni::CallStaticBooleanMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticBooleanMethodV")
    }

    pub extern "C" fn CallStaticBooleanMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jboolean {
        trace!(
            "jni::CallStaticBooleanMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticBooleanMethodA")
    }

    pub unsafe extern "C" fn CallStaticByteMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jbyte {
        trace!("jni::CallStaticByteMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticByteMethod")
    }

    pub extern "C" fn CallStaticByteMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jbyte {
        trace!(
            "jni::CallStaticByteMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticByteMethodV")
    }

    pub extern "C" fn CallStaticByteMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jbyte {
        trace!(
            "jni::CallStaticByteMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticByteMethodA")
    }

    pub unsafe extern "C" fn CallStaticCharMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jchar {
        trace!("jni::CallStaticCharMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticCharMethod")
    }

    pub extern "C" fn CallStaticCharMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jchar {
        trace!(
            "jni::CallStaticCharMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticCharMethodV")
    }

    pub extern "C" fn CallStaticCharMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jchar {
        trace!(
            "jni::CallStaticCharMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticCharMethodA")
    }

    pub unsafe extern "C" fn CallStaticShortMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jshort {
        trace!("jni::CallStaticShortMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticShortMethod")
    }

    pub extern "C" fn CallStaticShortMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jshort {
        trace!(
            "jni::CallStaticShortMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticShortMethodV")
    }

    pub extern "C" fn CallStaticShortMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jshort {
        trace!(
            "jni::CallStaticShortMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticShortMethodA")
    }

    pub unsafe extern "C" fn CallStaticIntMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jint {
        trace!("jni::CallStaticIntMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticIntMethod")
    }

    pub extern "C" fn CallStaticIntMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jint {
        trace!(
            "jni::CallStaticIntMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticIntMethodV")
    }

    pub extern "C" fn CallStaticIntMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jint {
        trace!(
            "jni::CallStaticIntMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticIntMethodA")
    }

    pub unsafe extern "C" fn CallStaticLongMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jlong {
        trace!("jni::CallStaticLongMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticLongMethod")
    }

    pub extern "C" fn CallStaticLongMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jlong {
        trace!(
            "jni::CallStaticLongMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticLongMethodV")
    }

    pub extern "C" fn CallStaticLongMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jlong {
        trace!(
            "jni::CallStaticLongMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticLongMethodA")
    }

    pub unsafe extern "C" fn CallStaticFloatMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jfloat {
        trace!("jni::CallStaticFloatMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticFloatMethod")
    }

    pub extern "C" fn CallStaticFloatMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jfloat {
        trace!(
            "jni::CallStaticFloatMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticFloatMethodV")
    }

    pub extern "C" fn CallStaticFloatMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jfloat {
        trace!(
            "jni::CallStaticFloatMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticFloatMethodA")
    }

    pub unsafe extern "C" fn CallStaticDoubleMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jdouble {
        trace!("jni::CallStaticDoubleMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticDoubleMethod")
    }

    pub extern "C" fn CallStaticDoubleMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jdouble {
        trace!(
            "jni::CallStaticDoubleMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticDoubleMethodV")
    }

    pub extern "C" fn CallStaticDoubleMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jdouble {
        trace!(
            "jni::CallStaticDoubleMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticDoubleMethodA")
    }

    pub unsafe extern "C" fn CallStaticVoidMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) {
        trace!("jni::CallStaticVoidMethod({:?}, {:?}, ...)", arg2, arg3);
        todo!("CallStaticVoidMethod")
    }

    pub extern "C" fn CallStaticVoidMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) {
        trace!(
            "jni::CallStaticVoidMethodV({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticVoidMethodV")
    }

    pub extern "C" fn CallStaticVoidMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) {
        trace!(
            "jni::CallStaticVoidMethodA({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("CallStaticVoidMethodA")
    }

    pub extern "C" fn GetStaticFieldID(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: *const ::std::os::raw::c_char,
        arg4: *const ::std::os::raw::c_char,
    ) -> jfieldID {
        trace!("jni::GetStaticFieldID({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("GetStaticFieldID")
    }

    pub extern "C" fn GetStaticObjectField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jobject {
        trace!("jni::GetStaticObjectField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticObjectField")
    }

    pub extern "C" fn GetStaticBooleanField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jboolean {
        trace!("jni::GetStaticBooleanField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticBooleanField")
    }

    pub extern "C" fn GetStaticByteField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jbyte {
        trace!("jni::GetStaticByteField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticByteField")
    }

    pub extern "C" fn GetStaticCharField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jchar {
        trace!("jni::GetStaticCharField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticCharField")
    }

    pub extern "C" fn GetStaticShortField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jshort {
        trace!("jni::GetStaticShortField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticShortField")
    }

    pub extern "C" fn GetStaticIntField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jint {
        trace!("jni::GetStaticIntField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticIntField")
    }

    pub extern "C" fn GetStaticLongField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jlong {
        trace!("jni::GetStaticLongField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticLongField")
    }

    pub extern "C" fn GetStaticFloatField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jfloat {
        trace!("jni::GetStaticFloatField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticFloatField")
    }

    pub extern "C" fn GetStaticDoubleField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jdouble {
        trace!("jni::GetStaticDoubleField({:?}, {:?})", arg2, arg3);
        todo!("GetStaticDoubleField")
    }

    pub extern "C" fn SetStaticObjectField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jobject,
    ) {
        trace!(
            "jni::SetStaticObjectField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticObjectField")
    }

    pub extern "C" fn SetStaticBooleanField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jboolean,
    ) {
        trace!(
            "jni::SetStaticBooleanField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticBooleanField")
    }

    pub extern "C" fn SetStaticByteField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jbyte,
    ) {
        trace!(
            "jni::SetStaticByteField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticByteField")
    }

    pub extern "C" fn SetStaticCharField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jchar,
    ) {
        trace!(
            "jni::SetStaticCharField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticCharField")
    }

    pub extern "C" fn SetStaticShortField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jshort,
    ) {
        trace!(
            "jni::SetStaticShortField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticShortField")
    }

    pub extern "C" fn SetStaticIntField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jint,
    ) {
        trace!("jni::SetStaticIntField({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("SetStaticIntField")
    }

    pub extern "C" fn SetStaticLongField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jlong,
    ) {
        trace!(
            "jni::SetStaticLongField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticLongField")
    }

    pub extern "C" fn SetStaticFloatField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jfloat,
    ) {
        trace!(
            "jni::SetStaticFloatField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticFloatField")
    }

    pub extern "C" fn SetStaticDoubleField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jdouble,
    ) {
        trace!(
            "jni::SetStaticDoubleField({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("SetStaticDoubleField")
    }

    pub extern "C" fn NewString(env: *mut JNIEnv, arg2: *const jchar, arg3: jsize) -> jstring {
        trace!("jni::NewString({:?}, {:?})", arg2, arg3);
        todo!("NewString")
    }

    pub extern "C" fn GetStringLength(env: *mut JNIEnv, arg2: jstring) -> jsize {
        trace!("jni::GetStringLength({:?})", arg2);
        todo!("GetStringLength")
    }

    pub extern "C" fn GetStringChars(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: *mut jboolean,
    ) -> *const jchar {
        trace!("jni::GetStringChars({:?}, {:?})", arg2, arg3);
        todo!("GetStringChars")
    }

    pub extern "C" fn ReleaseStringChars(env: *mut JNIEnv, arg2: jstring, arg3: *const jchar) {
        trace!("jni::ReleaseStringChars({:?}, {:?})", arg2, arg3);
        todo!("ReleaseStringChars")
    }

    pub extern "C" fn NewStringUTF(
        env: *mut JNIEnv,
        bytes: *const ::std::os::raw::c_char,
    ) -> jstring {
        trace!("jni::NewStringUTF({:?})", bytes);

        if bytes.is_null() {
            return jnull();
        }

        let cstr = unsafe { CStr::from_ptr(bytes) };
        let mstr = mstr::from_mutf8(cstr.to_bytes());

        match Object::new_string(mstr) {
            Ok(s) => vmref_into_raw(s) as jstring,
            Err(err) => {
                thread::get().set_exception(err.into());
                jnull()
            }
        }
    }

    pub extern "C" fn GetStringUTFLength(env: *mut JNIEnv, arg2: jstring) -> jsize {
        trace!("jni::GetStringUTFLength({:?})", arg2);
        todo!("GetStringUTFLength")
    }

    pub extern "C" fn GetStringUTFChars(
        env: *mut JNIEnv,
        string: jstring,
        is_copy: *mut jboolean,
    ) -> *const ::std::os::raw::c_char {
        trace!("GetStringUTFChars({:?})", string);
        let obj = unsafe { as_vmref::<Object>(string) };

        let chars = {
            // TODO this is gross
            let utf8 = obj.string_value_utf8().expect("not a string");
            let mutf8 = mstr::from_utf8(utf8.as_bytes())
                .into_owned()
                .into_boxed_mutf8_bytes()
                .into_vec();
            let cstring = unsafe { CString::from_vec_unchecked(mutf8) };
            cstring.into_raw()
        };

        // TODO store bytes in string directly?

        if !is_copy.is_null() {
            unsafe {
                // always copy lmao
                *is_copy = JNI_TRUE as u8;
            }
        }

        chars as *const _
    }

    pub extern "C" fn ReleaseStringUTFChars(
        env: *mut JNIEnv,
        string: jstring,
        utf: *const ::std::os::raw::c_char,
    ) {
        trace!("ReleaseStringUTFChars({:?})", string);
        let _cstring = unsafe { CString::from_raw(utf as *mut _) };
    }

    pub extern "C" fn GetArrayLength(env: *mut JNIEnv, arg2: jarray) -> jsize {
        trace!("jni::GetArrayLength({:?})", arg2);
        todo!("GetArrayLength")
    }

    pub extern "C" fn NewObjectArray(
        env: *mut JNIEnv,
        n: jsize,
        cls: jclass,
        default_val: jobject,
    ) -> jobjectArray {
        trace!("jni::NewObjectArray({:?}, {:?}, {:?})", n, cls, default_val);

        if is_null_throwing(cls) {
            return jnull();
        }
        let class = unsafe { as_vmref::<Class>(cls) };
        let default_val = DataValue::Reference(if default_val.is_null() {
            crate::class::null()
        } else {
            let obj = unsafe { as_vmref::<Object>(default_val) };
            VmRef::clone(&obj)
        });

        let t = thread::get();
        match t.exec_helper().collect_array(
            ArrayType::Reference(Arc::clone(&class)),
            repeat_n(Ok(default_val), n as usize),
        ) {
            Ok(arr) => vmref_into_raw(arr) as jobjectArray,
            Err(err) => {
                t.set_exception(err.into());
                jnull()
            }
        }
    }

    pub extern "C" fn GetObjectArrayElement(
        env: *mut JNIEnv,
        arg2: jobjectArray,
        arg3: jsize,
    ) -> jobject {
        trace!("jni::GetObjectArrayElement({:?}, {:?})", arg2, arg3);
        todo!("GetObjectArrayElement")
    }

    pub extern "C" fn SetObjectArrayElement(
        env: *mut JNIEnv,
        arr: jobjectArray,
        idx: jsize,
        obj: jobject,
    ) {
        trace!(
            "jni::SetObjectArrayElement({:?}, {:?}, {:?})",
            arr,
            idx,
            obj
        );

        let (array_obj, elem_obj) = unsafe { (as_vmref::<Object>(arr), as_vmref::<Object>(obj)) };

        if vmref_is_null(&array_obj) {
            return;
        }

        // check type
        if !array_obj
            .class_not_null()
            .can_array_be_assigned_to(&elem_obj)
        {
            thread::get().set_exception(Throwables::Other("java/lang/ArrayStoreException").into());
            return;
        }

        let mut array_backing = array_obj.array_unchecked();
        let val = match usize::try_from(idx)
            .ok()
            .and_then(|i| array_backing.get_mut(i))
        {
            Some(val) => val,
            None => {
                thread::get().set_exception(
                    Throwables::Other("java/lang/ArrayIndexOutOfBoundsException").into(),
                );
                return;
            }
        };

        *val = DataValue::Reference(VmRef::clone(&elem_obj));
    }

    pub extern "C" fn NewBooleanArray(env: *mut JNIEnv, arg2: jsize) -> jbooleanArray {
        trace!("jni::NewBooleanArray({:?})", arg2);
        todo!("NewBooleanArray")
    }

    pub extern "C" fn NewByteArray(env: *mut JNIEnv, arg2: jsize) -> jbyteArray {
        trace!("jni::NewByteArray({:?})", arg2);
        todo!("NewByteArray")
    }

    pub extern "C" fn NewCharArray(env: *mut JNIEnv, arg2: jsize) -> jcharArray {
        trace!("jni::NewCharArray({:?})", arg2);
        todo!("NewCharArray")
    }

    pub extern "C" fn NewShortArray(env: *mut JNIEnv, arg2: jsize) -> jshortArray {
        trace!("jni::NewShortArray({:?})", arg2);
        todo!("NewShortArray")
    }

    pub extern "C" fn NewIntArray(env: *mut JNIEnv, arg2: jsize) -> jintArray {
        trace!("jni::NewIntArray({:?})", arg2);
        todo!("NewIntArray")
    }

    pub extern "C" fn NewLongArray(env: *mut JNIEnv, arg2: jsize) -> jlongArray {
        trace!("jni::NewLongArray({:?})", arg2);
        todo!("NewLongArray")
    }

    pub extern "C" fn NewFloatArray(env: *mut JNIEnv, arg2: jsize) -> jfloatArray {
        trace!("jni::NewFloatArray({:?})", arg2);
        todo!("NewFloatArray")
    }

    pub extern "C" fn NewDoubleArray(env: *mut JNIEnv, arg2: jsize) -> jdoubleArray {
        trace!("jni::NewDoubleArray({:?})", arg2);
        todo!("NewDoubleArray")
    }

    pub extern "C" fn GetBooleanArrayElements(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: *mut jboolean,
    ) -> *mut jboolean {
        trace!("jni::GetBooleanArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetBooleanArrayElements")
    }

    pub extern "C" fn GetByteArrayElements(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: *mut jboolean,
    ) -> *mut jbyte {
        trace!("jni::GetByteArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetByteArrayElements")
    }

    pub extern "C" fn GetCharArrayElements(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: *mut jboolean,
    ) -> *mut jchar {
        trace!("jni::GetCharArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetCharArrayElements")
    }

    pub extern "C" fn GetShortArrayElements(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: *mut jboolean,
    ) -> *mut jshort {
        trace!("jni::GetShortArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetShortArrayElements")
    }

    pub extern "C" fn GetIntArrayElements(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: *mut jboolean,
    ) -> *mut jint {
        trace!("jni::GetIntArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetIntArrayElements")
    }

    pub extern "C" fn GetLongArrayElements(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: *mut jboolean,
    ) -> *mut jlong {
        trace!("jni::GetLongArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetLongArrayElements")
    }

    pub extern "C" fn GetFloatArrayElements(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: *mut jboolean,
    ) -> *mut jfloat {
        trace!("jni::GetFloatArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetFloatArrayElements")
    }

    pub extern "C" fn GetDoubleArrayElements(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: *mut jboolean,
    ) -> *mut jdouble {
        trace!("jni::GetDoubleArrayElements({:?}, {:?})", arg2, arg3);
        todo!("GetDoubleArrayElements")
    }

    pub extern "C" fn ReleaseBooleanArrayElements(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: *mut jboolean,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseBooleanArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseBooleanArrayElements")
    }

    pub extern "C" fn ReleaseByteArrayElements(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: *mut jbyte,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseByteArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseByteArrayElements")
    }

    pub extern "C" fn ReleaseCharArrayElements(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: *mut jchar,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseCharArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseCharArrayElements")
    }

    pub extern "C" fn ReleaseShortArrayElements(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: *mut jshort,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseShortArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseShortArrayElements")
    }

    pub extern "C" fn ReleaseIntArrayElements(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: *mut jint,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseIntArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseIntArrayElements")
    }

    pub extern "C" fn ReleaseLongArrayElements(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: *mut jlong,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseLongArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseLongArrayElements")
    }

    pub extern "C" fn ReleaseFloatArrayElements(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: *mut jfloat,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseFloatArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseFloatArrayElements")
    }

    pub extern "C" fn ReleaseDoubleArrayElements(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: *mut jdouble,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleaseDoubleArrayElements({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleaseDoubleArrayElements")
    }

    pub extern "C" fn GetBooleanArrayRegion(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jboolean,
    ) {
        trace!(
            "jni::GetBooleanArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetBooleanArrayRegion")
    }

    pub extern "C" fn GetByteArrayRegion(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jbyte,
    ) {
        trace!(
            "jni::GetByteArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetByteArrayRegion")
    }

    pub extern "C" fn GetCharArrayRegion(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jchar,
    ) {
        trace!(
            "jni::GetCharArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetCharArrayRegion")
    }

    pub extern "C" fn GetShortArrayRegion(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jshort,
    ) {
        trace!(
            "jni::GetShortArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetShortArrayRegion")
    }

    pub extern "C" fn GetIntArrayRegion(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jint,
    ) {
        trace!(
            "jni::GetIntArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetIntArrayRegion")
    }

    pub extern "C" fn GetLongArrayRegion(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jlong,
    ) {
        trace!(
            "jni::GetLongArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetLongArrayRegion")
    }

    pub extern "C" fn GetFloatArrayRegion(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jfloat,
    ) {
        trace!(
            "jni::GetFloatArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetFloatArrayRegion")
    }

    pub extern "C" fn GetDoubleArrayRegion(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jdouble,
    ) {
        trace!(
            "jni::GetDoubleArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetDoubleArrayRegion")
    }

    pub extern "C" fn SetBooleanArrayRegion(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jboolean,
    ) {
        trace!(
            "jni::SetBooleanArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetBooleanArrayRegion")
    }

    pub extern "C" fn SetByteArrayRegion(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jbyte,
    ) {
        trace!(
            "jni::SetByteArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetByteArrayRegion")
    }

    pub extern "C" fn SetCharArrayRegion(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jchar,
    ) {
        trace!(
            "jni::SetCharArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetCharArrayRegion")
    }

    pub extern "C" fn SetShortArrayRegion(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jshort,
    ) {
        trace!(
            "jni::SetShortArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetShortArrayRegion")
    }

    pub extern "C" fn SetIntArrayRegion(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jint,
    ) {
        trace!(
            "jni::SetIntArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetIntArrayRegion")
    }

    pub extern "C" fn SetLongArrayRegion(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jlong,
    ) {
        trace!(
            "jni::SetLongArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetLongArrayRegion")
    }

    pub extern "C" fn SetFloatArrayRegion(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jfloat,
    ) {
        trace!(
            "jni::SetFloatArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetFloatArrayRegion")
    }

    pub extern "C" fn SetDoubleArrayRegion(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jdouble,
    ) {
        trace!(
            "jni::SetDoubleArrayRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("SetDoubleArrayRegion")
    }

    pub extern "C" fn RegisterNatives(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: *const JNINativeMethod,
        arg4: jint,
    ) -> jint {
        trace!("jni::RegisterNatives({:?}, {:?}, {:?})", arg2, arg3, arg4);
        todo!("RegisterNatives")
    }

    pub extern "C" fn UnregisterNatives(env: *mut JNIEnv, arg2: jclass) -> jint {
        trace!("jni::UnregisterNatives({:?})", arg2);
        todo!("UnregisterNatives")
    }

    pub extern "C" fn MonitorEnter(env: *mut JNIEnv, arg2: jobject) -> jint {
        trace!("jni::MonitorEnter({:?})", arg2);
        todo!("MonitorEnter")
    }

    pub extern "C" fn MonitorExit(env: *mut JNIEnv, arg2: jobject) -> jint {
        trace!("jni::MonitorExit({:?})", arg2);
        todo!("MonitorExit")
    }

    pub extern "C" fn GetJavaVM(env: *mut JNIEnv, arg2: *mut *mut JavaVM) -> jint {
        trace!("jni::GetJavaVM({:?})", arg2);
        todo!("GetJavaVM")
    }

    pub extern "C" fn GetStringRegion(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jchar,
    ) {
        trace!(
            "jni::GetStringRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetStringRegion")
    }

    pub extern "C" fn GetStringUTFRegion(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut ::std::os::raw::c_char,
    ) {
        trace!(
            "jni::GetStringUTFRegion({:?}, {:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4,
            arg5
        );
        todo!("GetStringUTFRegion")
    }

    pub extern "C" fn GetPrimitiveArrayCritical(
        env: *mut JNIEnv,
        arg2: jarray,
        arg3: *mut jboolean,
    ) -> *mut ::std::os::raw::c_void {
        trace!("jni::GetPrimitiveArrayCritical({:?}, {:?})", arg2, arg3);
        todo!("GetPrimitiveArrayCritical")
    }

    pub extern "C" fn ReleasePrimitiveArrayCritical(
        env: *mut JNIEnv,
        arg2: jarray,
        arg3: *mut ::std::os::raw::c_void,
        arg4: jint,
    ) {
        trace!(
            "jni::ReleasePrimitiveArrayCritical({:?}, {:?}, {:?})",
            arg2,
            arg3,
            arg4
        );
        todo!("ReleasePrimitiveArrayCritical")
    }

    pub extern "C" fn GetStringCritical(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: *mut jboolean,
    ) -> *const jchar {
        trace!("jni::GetStringCritical({:?}, {:?})", arg2, arg3);
        todo!("GetStringCritical")
    }

    pub extern "C" fn ReleaseStringCritical(env: *mut JNIEnv, arg2: jstring, arg3: *const jchar) {
        trace!("jni::ReleaseStringCritical({:?}, {:?})", arg2, arg3);
        todo!("ReleaseStringCritical")
    }

    pub extern "C" fn NewWeakGlobalRef(env: *mut JNIEnv, arg2: jobject) -> jweak {
        trace!("jni::NewWeakGlobalRef({:?})", arg2);
        todo!("NewWeakGlobalRef")
    }

    pub extern "C" fn DeleteWeakGlobalRef(env: *mut JNIEnv, arg2: jweak) {
        trace!("jni::DeleteWeakGlobalRef({:?})", arg2);
        todo!("DeleteWeakGlobalRef")
    }

    pub extern "C" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
        trace!("jni::ExceptionCheck()");
        todo!("ExceptionCheck")
    }

    pub extern "C" fn NewDirectByteBuffer(
        env: *mut JNIEnv,
        arg2: *mut ::std::os::raw::c_void,
        arg3: jlong,
    ) -> jobject {
        trace!("jni::NewDirectByteBuffer({:?}, {:?})", arg2, arg3);
        todo!("NewDirectByteBuffer")
    }

    pub extern "C" fn GetDirectBufferAddress(
        env: *mut JNIEnv,
        arg2: jobject,
    ) -> *mut ::std::os::raw::c_void {
        trace!("jni::GetDirectBufferAddress({:?})", arg2);
        todo!("GetDirectBufferAddress")
    }

    pub extern "C" fn GetDirectBufferCapacity(env: *mut JNIEnv, arg2: jobject) -> jlong {
        trace!("jni::GetDirectBufferCapacity({:?})", arg2);
        todo!("GetDirectBufferCapacity")
    }

    pub extern "C" fn GetObjectRefType(env: *mut JNIEnv, arg2: jobject) -> jobjectRefType {
        trace!("jni::GetObjectRefType({:?})", arg2);
        todo!("GetObjectRefType")
    }
}

impl From<JniFieldId> for sys::jfieldID {
    fn from(f: JniFieldId) -> Self {
        let id = f.0.get();
        debug_assert!(std::mem::size_of_val(&id) <= std::mem::size_of::<sys::jfieldID>());
        id as jfieldID
    }
}
impl From<sys::jfieldID> for JniFieldId {
    fn from(f: jfieldID) -> Self {
        let id = f as u32;
        JniFieldId(FieldId::from_raw(id))
    }
}

impl From<JniMethodId> for sys::jmethodID {
    fn from(m: JniMethodId) -> Self {
        vmref_into_raw(m.0) as sys::jmethodID
    }
}

impl From<sys::jmethodID> for JniMethodId {
    fn from(m: jmethodID) -> Self {
        let method = unsafe { vmref_from_raw::<Method>(m as *const _) };
        JniMethodId(method)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jni::sys::jfieldID;

    #[test]
    fn from_jfieldid() {
        assert!(std::mem::size_of::<JniFieldId>() <= std::mem::size_of::<jfieldID>());
    }
}
