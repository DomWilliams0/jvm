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
    use std::ptr;

    pub extern "C" fn DestroyJavaVM(arg1: *mut JavaVM) -> jint {
        todo!("DestroyJavaVM")
    }

    pub extern "C" fn AttachCurrentThread(
        arg1: *mut JavaVM,
        arg2: *mut *mut ::std::os::raw::c_void,
        arg3: *mut ::std::os::raw::c_void,
    ) -> jint {
        todo!("AttachCurrentThread")
    }

    pub extern "C" fn DetachCurrentThread(arg1: *mut JavaVM) -> jint {
        todo!("DetachCurrentThread")
    }

    pub extern "C" fn GetEnv(
        vm: *mut JavaVM,
        env_out: *mut *mut ::std::os::raw::c_void,
        version: jint,
    ) -> jint {
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
        todo!("AttachCurrentThreadAsDaemon")
    }
}

mod jnienv {
    #![allow(non_snake_case, unused_variables)]
    use crate::alloc::{vmref_from_raw, vmref_increment, vmref_into_raw, vmref_ptr, VmRef};
    use crate::class::{Class, WhichLoader};

    use crate::jni::api::{JniFieldId, JniMethodId};
    use crate::jni::sys::*;
    use crate::jni::JNI_VERSION;
    use crate::thread;
    use crate::types::DataType;
    use cafebabe::mutf8::mstr;
    use cafebabe::MethodAccessFlags;
    use log::*;
    use std::ffi::CStr;
    use std::mem::ManuallyDrop;
    use std::ptr;

    unsafe fn as_string<'a>(s: *const ::std::os::raw::c_char) -> &'a mstr {
        let cstr = CStr::from_ptr(s);
        mstr::from_mutf8(cstr.to_bytes())
    }

    unsafe fn as_vmref<T>(obj: *const ::std::os::raw::c_void) -> ManuallyDrop<VmRef<T>> {
        ManuallyDrop::new(vmref_from_raw(obj as *const T))
    }

    pub extern "C" fn GetVersion(env: *mut JNIEnv) -> jint {
        JNI_VERSION as jint
    }

    pub extern "C" fn DefineClass(
        env: *mut JNIEnv,
        arg2: *const ::std::os::raw::c_char,
        arg3: jobject,
        arg4: *const jbyte,
        arg5: jsize,
    ) -> jclass {
        todo!("DefineClass")
    }

    pub extern "C" fn FindClass(env: *mut JNIEnv, name: *const ::std::os::raw::c_char) -> jclass {
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
                interp.with_current_jni_frame(|jni| jni.add_local_ref(&cls));

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
        todo!("FromReflectedMethod")
    }

    pub extern "C" fn FromReflectedField(env: *mut JNIEnv, arg2: jobject) -> jfieldID {
        todo!("FromReflectedField")
    }

    pub extern "C" fn ToReflectedMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: jboolean,
    ) -> jobject {
        todo!("ToReflectedMethod")
    }

    pub extern "C" fn GetSuperclass(env: *mut JNIEnv, arg2: jclass) -> jclass {
        todo!("GetSuperclass")
    }

    pub extern "C" fn IsAssignableFrom(env: *mut JNIEnv, arg2: jclass, arg3: jclass) -> jboolean {
        todo!("IsAssignableFrom")
    }

    pub extern "C" fn ToReflectedField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jboolean,
    ) -> jobject {
        todo!("ToReflectedField")
    }

    pub extern "C" fn Throw(env: *mut JNIEnv, arg2: jthrowable) -> jint {
        todo!("Throw")
    }

    pub extern "C" fn ThrowNew(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: *const ::std::os::raw::c_char,
    ) -> jint {
        todo!("ThrowNew")
    }

    pub extern "C" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
        todo!("ExceptionOccurred")
    }

    pub extern "C" fn ExceptionDescribe(env: *mut JNIEnv) {
        todo!("ExceptionDescribe")
    }

    pub extern "C" fn ExceptionClear(env: *mut JNIEnv) {
        todo!("ExceptionClear")
    }

    pub extern "C" fn FatalError(env: *mut JNIEnv, arg2: *const ::std::os::raw::c_char) {
        todo!("FatalError")
    }

    pub extern "C" fn PushLocalFrame(env: *mut JNIEnv, arg2: jint) -> jint {
        todo!("PushLocalFrame")
    }

    pub extern "C" fn PopLocalFrame(env: *mut JNIEnv, arg2: jobject) -> jobject {
        todo!("PopLocalFrame")
    }

    pub extern "C" fn NewGlobalRef(env: *mut JNIEnv, obj: jobject) -> jobject {
        // TODO keep track of global references in jvm or is it ok to leak them like this?

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
        todo!("DeleteGlobalRef")
    }

    pub extern "C" fn DeleteLocalRef(env: *mut JNIEnv, arg2: jobject) {
        todo!("DeleteLocalRef")
    }

    pub extern "C" fn IsSameObject(env: *mut JNIEnv, arg2: jobject, arg3: jobject) -> jboolean {
        todo!("IsSameObject")
    }

    pub extern "C" fn NewLocalRef(env: *mut JNIEnv, arg2: jobject) -> jobject {
        todo!("NewLocalRef")
    }

    pub extern "C" fn EnsureLocalCapacity(env: *mut JNIEnv, arg2: jint) -> jint {
        todo!("EnsureLocalCapacity")
    }

    pub extern "C" fn AllocObject(env: *mut JNIEnv, arg2: jclass) -> jobject {
        todo!("AllocObject")
    }

    pub unsafe extern "C" fn NewObject(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jobject {
        todo!("NewObject")
    }

    pub extern "C" fn NewObjectV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jobject {
        todo!("NewObjectV")
    }

    pub extern "C" fn NewObjectA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jobject {
        todo!("NewObjectA")
    }

    pub extern "C" fn GetObjectClass(env: *mut JNIEnv, arg2: jobject) -> jclass {
        todo!("GetObjectClass")
    }

    pub extern "C" fn IsInstanceOf(env: *mut JNIEnv, arg2: jobject, arg3: jclass) -> jboolean {
        todo!("IsInstanceOf")
    }

    pub extern "C" fn GetMethodID(
        env: *mut JNIEnv,
        class: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> jmethodID {
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
            "GetMethodId({:?}::{} (type {:?})) -> {:?}",
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
        todo!("CallObjectMethod")
    }

    pub extern "C" fn CallObjectMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jobject {
        todo!("CallObjectMethodV")
    }

    pub extern "C" fn CallObjectMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jobject {
        todo!("CallObjectMethodA")
    }

    pub unsafe extern "C" fn CallBooleanMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jboolean {
        todo!("CallBooleanMethod")
    }

    pub extern "C" fn CallBooleanMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jboolean {
        todo!("CallBooleanMethodV")
    }

    pub extern "C" fn CallBooleanMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jboolean {
        todo!("CallBooleanMethodA")
    }

    pub unsafe extern "C" fn CallByteMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jbyte {
        todo!("CallByteMethod")
    }

    pub extern "C" fn CallByteMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jbyte {
        todo!("CallByteMethodV")
    }

    pub extern "C" fn CallByteMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jbyte {
        todo!("CallByteMethodA")
    }

    pub unsafe extern "C" fn CallCharMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jchar {
        todo!("CallCharMethod")
    }

    pub extern "C" fn CallCharMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jchar {
        todo!("CallCharMethodV")
    }

    pub extern "C" fn CallCharMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jchar {
        todo!("CallCharMethodA")
    }

    pub unsafe extern "C" fn CallShortMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jshort {
        todo!("CallShortMethod")
    }

    pub extern "C" fn CallShortMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jshort {
        todo!("CallShortMethodV")
    }

    pub extern "C" fn CallShortMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jshort {
        todo!("CallShortMethodA")
    }

    pub unsafe extern "C" fn CallIntMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jint {
        todo!("CallIntMethod")
    }

    pub extern "C" fn CallIntMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jint {
        todo!("CallIntMethodV")
    }

    pub extern "C" fn CallIntMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jint {
        todo!("CallIntMethodA")
    }

    pub unsafe extern "C" fn CallLongMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jlong {
        todo!("CallLongMethod")
    }

    pub extern "C" fn CallLongMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jlong {
        todo!("CallLongMethodV")
    }

    pub extern "C" fn CallLongMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jlong {
        todo!("CallLongMethodA")
    }

    pub unsafe extern "C" fn CallFloatMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jfloat {
        todo!("CallFloatMethod")
    }

    pub extern "C" fn CallFloatMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jfloat {
        todo!("CallFloatMethodV")
    }

    pub extern "C" fn CallFloatMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jfloat {
        todo!("CallFloatMethodA")
    }

    pub unsafe extern "C" fn CallDoubleMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        ...
    ) -> jdouble {
        todo!("CallDoubleMethod")
    }

    pub extern "C" fn CallDoubleMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jdouble {
        todo!("CallDoubleMethodV")
    }

    pub extern "C" fn CallDoubleMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jdouble {
        todo!("CallDoubleMethodA")
    }

    pub unsafe extern "C" fn CallVoidMethod(env: *mut JNIEnv, arg2: jobject, arg3: jmethodID, ...) {
        todo!("CallVoidMethod")
    }

    pub extern "C" fn CallVoidMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) {
        todo!("CallVoidMethodV")
    }

    pub extern "C" fn CallVoidMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) {
        todo!("CallVoidMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualObjectMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jobject {
        todo!("CallNonvirtualObjectMethod")
    }

    pub extern "C" fn CallNonvirtualObjectMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jobject {
        todo!("CallNonvirtualObjectMethodV")
    }

    pub extern "C" fn CallNonvirtualObjectMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jobject {
        todo!("CallNonvirtualObjectMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualBooleanMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jboolean {
        todo!("CallNonvirtualBooleanMethod")
    }

    pub extern "C" fn CallNonvirtualBooleanMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jboolean {
        todo!("CallNonvirtualBooleanMethodV")
    }

    pub extern "C" fn CallNonvirtualBooleanMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jboolean {
        todo!("CallNonvirtualBooleanMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualByteMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jbyte {
        todo!("CallNonvirtualByteMethod")
    }

    pub extern "C" fn CallNonvirtualByteMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jbyte {
        todo!("CallNonvirtualByteMethodV")
    }

    pub extern "C" fn CallNonvirtualByteMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jbyte {
        todo!("CallNonvirtualByteMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualCharMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jchar {
        todo!("CallNonvirtualCharMethod")
    }

    pub extern "C" fn CallNonvirtualCharMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jchar {
        todo!("CallNonvirtualCharMethodV")
    }

    pub extern "C" fn CallNonvirtualCharMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jchar {
        todo!("CallNonvirtualCharMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualShortMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jshort {
        todo!("CallNonvirtualShortMethod")
    }

    pub extern "C" fn CallNonvirtualShortMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jshort {
        todo!("CallNonvirtualShortMethodV")
    }

    pub extern "C" fn CallNonvirtualShortMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jshort {
        todo!("CallNonvirtualShortMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualIntMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jint {
        todo!("CallNonvirtualIntMethod")
    }

    pub extern "C" fn CallNonvirtualIntMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jint {
        todo!("CallNonvirtualIntMethodV")
    }

    pub extern "C" fn CallNonvirtualIntMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jint {
        todo!("CallNonvirtualIntMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualLongMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jlong {
        todo!("CallNonvirtualLongMethod")
    }

    pub extern "C" fn CallNonvirtualLongMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jlong {
        todo!("CallNonvirtualLongMethodV")
    }

    pub extern "C" fn CallNonvirtualLongMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jlong {
        todo!("CallNonvirtualLongMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualFloatMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jfloat {
        todo!("CallNonvirtualFloatMethod")
    }

    pub extern "C" fn CallNonvirtualFloatMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jfloat {
        todo!("CallNonvirtualFloatMethodV")
    }

    pub extern "C" fn CallNonvirtualFloatMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jfloat {
        todo!("CallNonvirtualFloatMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualDoubleMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) -> jdouble {
        todo!("CallNonvirtualDoubleMethod")
    }

    pub extern "C" fn CallNonvirtualDoubleMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) -> jdouble {
        todo!("CallNonvirtualDoubleMethodV")
    }

    pub extern "C" fn CallNonvirtualDoubleMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) -> jdouble {
        todo!("CallNonvirtualDoubleMethodA")
    }

    pub unsafe extern "C" fn CallNonvirtualVoidMethod(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        ...
    ) {
        todo!("CallNonvirtualVoidMethod")
    }

    pub extern "C" fn CallNonvirtualVoidMethodV(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *mut __va_list_tag,
    ) {
        todo!("CallNonvirtualVoidMethodV")
    }

    pub extern "C" fn CallNonvirtualVoidMethodA(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jclass,
        arg4: jmethodID,
        arg5: *const jvalue,
    ) {
        todo!("CallNonvirtualVoidMethodA")
    }

    pub extern "C" fn GetFieldID(
        env: *mut JNIEnv,
        class: jclass,
        name: *const ::std::os::raw::c_char,
        sig: *const ::std::os::raw::c_char,
    ) -> jfieldID {
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
        todo!("GetObjectField")
    }

    pub extern "C" fn GetBooleanField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jboolean {
        todo!("GetBooleanField")
    }

    pub extern "C" fn GetByteField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jbyte {
        todo!("GetByteField")
    }

    pub extern "C" fn GetCharField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jchar {
        todo!("GetCharField")
    }

    pub extern "C" fn GetShortField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jshort {
        todo!("GetShortField")
    }

    pub extern "C" fn GetIntField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jint {
        todo!("GetIntField")
    }

    pub extern "C" fn GetLongField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jlong {
        todo!("GetLongField")
    }

    pub extern "C" fn GetFloatField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jfloat {
        todo!("GetFloatField")
    }

    pub extern "C" fn GetDoubleField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID) -> jdouble {
        todo!("GetDoubleField")
    }

    pub extern "C" fn SetObjectField(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jfieldID,
        arg4: jobject,
    ) {
        todo!("SetObjectField")
    }

    pub extern "C" fn SetBooleanField(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jfieldID,
        arg4: jboolean,
    ) {
        todo!("SetBooleanField")
    }

    pub extern "C" fn SetByteField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jbyte) {
        todo!("SetByteField")
    }

    pub extern "C" fn SetCharField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jchar) {
        todo!("SetCharField")
    }

    pub extern "C" fn SetShortField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jshort) {
        todo!("SetShortField")
    }

    pub extern "C" fn SetIntField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jint) {
        todo!("SetIntField")
    }

    pub extern "C" fn SetLongField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jlong) {
        todo!("SetLongField")
    }

    pub extern "C" fn SetFloatField(env: *mut JNIEnv, arg2: jobject, arg3: jfieldID, arg4: jfloat) {
        todo!("SetFloatField")
    }

    pub extern "C" fn SetDoubleField(
        env: *mut JNIEnv,
        arg2: jobject,
        arg3: jfieldID,
        arg4: jdouble,
    ) {
        todo!("SetDoubleField")
    }

    pub extern "C" fn GetStaticMethodID(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: *const ::std::os::raw::c_char,
        arg4: *const ::std::os::raw::c_char,
    ) -> jmethodID {
        todo!("GetStaticMethodID")
    }

    pub unsafe extern "C" fn CallStaticObjectMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jobject {
        todo!("CallStaticObjectMethod")
    }

    pub extern "C" fn CallStaticObjectMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jobject {
        todo!("CallStaticObjectMethodV")
    }

    pub extern "C" fn CallStaticObjectMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jobject {
        todo!("CallStaticObjectMethodA")
    }

    pub unsafe extern "C" fn CallStaticBooleanMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jboolean {
        todo!("CallStaticBooleanMethod")
    }

    pub extern "C" fn CallStaticBooleanMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jboolean {
        todo!("CallStaticBooleanMethodV")
    }

    pub extern "C" fn CallStaticBooleanMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jboolean {
        todo!("CallStaticBooleanMethodA")
    }

    pub unsafe extern "C" fn CallStaticByteMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jbyte {
        todo!("CallStaticByteMethod")
    }

    pub extern "C" fn CallStaticByteMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jbyte {
        todo!("CallStaticByteMethodV")
    }

    pub extern "C" fn CallStaticByteMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jbyte {
        todo!("CallStaticByteMethodA")
    }

    pub unsafe extern "C" fn CallStaticCharMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jchar {
        todo!("CallStaticCharMethod")
    }

    pub extern "C" fn CallStaticCharMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jchar {
        todo!("CallStaticCharMethodV")
    }

    pub extern "C" fn CallStaticCharMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jchar {
        todo!("CallStaticCharMethodA")
    }

    pub unsafe extern "C" fn CallStaticShortMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jshort {
        todo!("CallStaticShortMethod")
    }

    pub extern "C" fn CallStaticShortMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jshort {
        todo!("CallStaticShortMethodV")
    }

    pub extern "C" fn CallStaticShortMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jshort {
        todo!("CallStaticShortMethodA")
    }

    pub unsafe extern "C" fn CallStaticIntMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jint {
        todo!("CallStaticIntMethod")
    }

    pub extern "C" fn CallStaticIntMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jint {
        todo!("CallStaticIntMethodV")
    }

    pub extern "C" fn CallStaticIntMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jint {
        todo!("CallStaticIntMethodA")
    }

    pub unsafe extern "C" fn CallStaticLongMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jlong {
        todo!("CallStaticLongMethod")
    }

    pub extern "C" fn CallStaticLongMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jlong {
        todo!("CallStaticLongMethodV")
    }

    pub extern "C" fn CallStaticLongMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jlong {
        todo!("CallStaticLongMethodA")
    }

    pub unsafe extern "C" fn CallStaticFloatMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jfloat {
        todo!("CallStaticFloatMethod")
    }

    pub extern "C" fn CallStaticFloatMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jfloat {
        todo!("CallStaticFloatMethodV")
    }

    pub extern "C" fn CallStaticFloatMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jfloat {
        todo!("CallStaticFloatMethodA")
    }

    pub unsafe extern "C" fn CallStaticDoubleMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) -> jdouble {
        todo!("CallStaticDoubleMethod")
    }

    pub extern "C" fn CallStaticDoubleMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) -> jdouble {
        todo!("CallStaticDoubleMethodV")
    }

    pub extern "C" fn CallStaticDoubleMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) -> jdouble {
        todo!("CallStaticDoubleMethodA")
    }

    pub unsafe extern "C" fn CallStaticVoidMethod(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        ...
    ) {
        todo!("CallStaticVoidMethod")
    }

    pub extern "C" fn CallStaticVoidMethodV(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *mut __va_list_tag,
    ) {
        todo!("CallStaticVoidMethodV")
    }

    pub extern "C" fn CallStaticVoidMethodA(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jmethodID,
        arg4: *const jvalue,
    ) {
        todo!("CallStaticVoidMethodA")
    }

    pub extern "C" fn GetStaticFieldID(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: *const ::std::os::raw::c_char,
        arg4: *const ::std::os::raw::c_char,
    ) -> jfieldID {
        todo!("GetStaticFieldID")
    }

    pub extern "C" fn GetStaticObjectField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jobject {
        todo!("GetStaticObjectField")
    }

    pub extern "C" fn GetStaticBooleanField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jboolean {
        todo!("GetStaticBooleanField")
    }

    pub extern "C" fn GetStaticByteField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jbyte {
        todo!("GetStaticByteField")
    }

    pub extern "C" fn GetStaticCharField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jchar {
        todo!("GetStaticCharField")
    }

    pub extern "C" fn GetStaticShortField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jshort {
        todo!("GetStaticShortField")
    }

    pub extern "C" fn GetStaticIntField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jint {
        todo!("GetStaticIntField")
    }

    pub extern "C" fn GetStaticLongField(env: *mut JNIEnv, arg2: jclass, arg3: jfieldID) -> jlong {
        todo!("GetStaticLongField")
    }

    pub extern "C" fn GetStaticFloatField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jfloat {
        todo!("GetStaticFloatField")
    }

    pub extern "C" fn GetStaticDoubleField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
    ) -> jdouble {
        todo!("GetStaticDoubleField")
    }

    pub extern "C" fn SetStaticObjectField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jobject,
    ) {
        todo!("SetStaticObjectField")
    }

    pub extern "C" fn SetStaticBooleanField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jboolean,
    ) {
        todo!("SetStaticBooleanField")
    }

    pub extern "C" fn SetStaticByteField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jbyte,
    ) {
        todo!("SetStaticByteField")
    }

    pub extern "C" fn SetStaticCharField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jchar,
    ) {
        todo!("SetStaticCharField")
    }

    pub extern "C" fn SetStaticShortField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jshort,
    ) {
        todo!("SetStaticShortField")
    }

    pub extern "C" fn SetStaticIntField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jint,
    ) {
        todo!("SetStaticIntField")
    }

    pub extern "C" fn SetStaticLongField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jlong,
    ) {
        todo!("SetStaticLongField")
    }

    pub extern "C" fn SetStaticFloatField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jfloat,
    ) {
        todo!("SetStaticFloatField")
    }

    pub extern "C" fn SetStaticDoubleField(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: jfieldID,
        arg4: jdouble,
    ) {
        todo!("SetStaticDoubleField")
    }

    pub extern "C" fn NewString(env: *mut JNIEnv, arg2: *const jchar, arg3: jsize) -> jstring {
        todo!("NewString")
    }

    pub extern "C" fn GetStringLength(env: *mut JNIEnv, arg2: jstring) -> jsize {
        todo!("GetStringLength")
    }

    pub extern "C" fn GetStringChars(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: *mut jboolean,
    ) -> *const jchar {
        todo!("GetStringChars")
    }

    pub extern "C" fn ReleaseStringChars(env: *mut JNIEnv, arg2: jstring, arg3: *const jchar) {
        todo!("ReleaseStringChars")
    }

    pub extern "C" fn NewStringUTF(
        env: *mut JNIEnv,
        arg2: *const ::std::os::raw::c_char,
    ) -> jstring {
        todo!("NewStringUTF")
    }

    pub extern "C" fn GetStringUTFLength(env: *mut JNIEnv, arg2: jstring) -> jsize {
        todo!("GetStringUTFLength")
    }

    pub extern "C" fn GetStringUTFChars(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: *mut jboolean,
    ) -> *const ::std::os::raw::c_char {
        todo!("GetStringUTFChars")
    }

    pub extern "C" fn ReleaseStringUTFChars(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: *const ::std::os::raw::c_char,
    ) {
        todo!("ReleaseStringUTFChars")
    }

    pub extern "C" fn GetArrayLength(env: *mut JNIEnv, arg2: jarray) -> jsize {
        todo!("GetArrayLength")
    }

    pub extern "C" fn NewObjectArray(
        env: *mut JNIEnv,
        arg2: jsize,
        arg3: jclass,
        arg4: jobject,
    ) -> jobjectArray {
        todo!("NewObjectArray")
    }

    pub extern "C" fn GetObjectArrayElement(
        env: *mut JNIEnv,
        arg2: jobjectArray,
        arg3: jsize,
    ) -> jobject {
        todo!("GetObjectArrayElement")
    }

    pub extern "C" fn SetObjectArrayElement(
        env: *mut JNIEnv,
        arg2: jobjectArray,
        arg3: jsize,
        arg4: jobject,
    ) {
        todo!("SetObjectArrayElement")
    }

    pub extern "C" fn NewBooleanArray(env: *mut JNIEnv, arg2: jsize) -> jbooleanArray {
        todo!("NewBooleanArray")
    }

    pub extern "C" fn NewByteArray(env: *mut JNIEnv, arg2: jsize) -> jbyteArray {
        todo!("NewByteArray")
    }

    pub extern "C" fn NewCharArray(env: *mut JNIEnv, arg2: jsize) -> jcharArray {
        todo!("NewCharArray")
    }

    pub extern "C" fn NewShortArray(env: *mut JNIEnv, arg2: jsize) -> jshortArray {
        todo!("NewShortArray")
    }

    pub extern "C" fn NewIntArray(env: *mut JNIEnv, arg2: jsize) -> jintArray {
        todo!("NewIntArray")
    }

    pub extern "C" fn NewLongArray(env: *mut JNIEnv, arg2: jsize) -> jlongArray {
        todo!("NewLongArray")
    }

    pub extern "C" fn NewFloatArray(env: *mut JNIEnv, arg2: jsize) -> jfloatArray {
        todo!("NewFloatArray")
    }

    pub extern "C" fn NewDoubleArray(env: *mut JNIEnv, arg2: jsize) -> jdoubleArray {
        todo!("NewDoubleArray")
    }

    pub extern "C" fn GetBooleanArrayElements(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: *mut jboolean,
    ) -> *mut jboolean {
        todo!("GetBooleanArrayElements")
    }

    pub extern "C" fn GetByteArrayElements(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: *mut jboolean,
    ) -> *mut jbyte {
        todo!("GetByteArrayElements")
    }

    pub extern "C" fn GetCharArrayElements(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: *mut jboolean,
    ) -> *mut jchar {
        todo!("GetCharArrayElements")
    }

    pub extern "C" fn GetShortArrayElements(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: *mut jboolean,
    ) -> *mut jshort {
        todo!("GetShortArrayElements")
    }

    pub extern "C" fn GetIntArrayElements(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: *mut jboolean,
    ) -> *mut jint {
        todo!("GetIntArrayElements")
    }

    pub extern "C" fn GetLongArrayElements(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: *mut jboolean,
    ) -> *mut jlong {
        todo!("GetLongArrayElements")
    }

    pub extern "C" fn GetFloatArrayElements(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: *mut jboolean,
    ) -> *mut jfloat {
        todo!("GetFloatArrayElements")
    }

    pub extern "C" fn GetDoubleArrayElements(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: *mut jboolean,
    ) -> *mut jdouble {
        todo!("GetDoubleArrayElements")
    }

    pub extern "C" fn ReleaseBooleanArrayElements(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: *mut jboolean,
        arg4: jint,
    ) {
        todo!("ReleaseBooleanArrayElements")
    }

    pub extern "C" fn ReleaseByteArrayElements(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: *mut jbyte,
        arg4: jint,
    ) {
        todo!("ReleaseByteArrayElements")
    }

    pub extern "C" fn ReleaseCharArrayElements(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: *mut jchar,
        arg4: jint,
    ) {
        todo!("ReleaseCharArrayElements")
    }

    pub extern "C" fn ReleaseShortArrayElements(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: *mut jshort,
        arg4: jint,
    ) {
        todo!("ReleaseShortArrayElements")
    }

    pub extern "C" fn ReleaseIntArrayElements(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: *mut jint,
        arg4: jint,
    ) {
        todo!("ReleaseIntArrayElements")
    }

    pub extern "C" fn ReleaseLongArrayElements(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: *mut jlong,
        arg4: jint,
    ) {
        todo!("ReleaseLongArrayElements")
    }

    pub extern "C" fn ReleaseFloatArrayElements(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: *mut jfloat,
        arg4: jint,
    ) {
        todo!("ReleaseFloatArrayElements")
    }

    pub extern "C" fn ReleaseDoubleArrayElements(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: *mut jdouble,
        arg4: jint,
    ) {
        todo!("ReleaseDoubleArrayElements")
    }

    pub extern "C" fn GetBooleanArrayRegion(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jboolean,
    ) {
        todo!("GetBooleanArrayRegion")
    }

    pub extern "C" fn GetByteArrayRegion(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jbyte,
    ) {
        todo!("GetByteArrayRegion")
    }

    pub extern "C" fn GetCharArrayRegion(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jchar,
    ) {
        todo!("GetCharArrayRegion")
    }

    pub extern "C" fn GetShortArrayRegion(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jshort,
    ) {
        todo!("GetShortArrayRegion")
    }

    pub extern "C" fn GetIntArrayRegion(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jint,
    ) {
        todo!("GetIntArrayRegion")
    }

    pub extern "C" fn GetLongArrayRegion(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jlong,
    ) {
        todo!("GetLongArrayRegion")
    }

    pub extern "C" fn GetFloatArrayRegion(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jfloat,
    ) {
        todo!("GetFloatArrayRegion")
    }

    pub extern "C" fn GetDoubleArrayRegion(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jdouble,
    ) {
        todo!("GetDoubleArrayRegion")
    }

    pub extern "C" fn SetBooleanArrayRegion(
        env: *mut JNIEnv,
        arg2: jbooleanArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jboolean,
    ) {
        todo!("SetBooleanArrayRegion")
    }

    pub extern "C" fn SetByteArrayRegion(
        env: *mut JNIEnv,
        arg2: jbyteArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jbyte,
    ) {
        todo!("SetByteArrayRegion")
    }

    pub extern "C" fn SetCharArrayRegion(
        env: *mut JNIEnv,
        arg2: jcharArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jchar,
    ) {
        todo!("SetCharArrayRegion")
    }

    pub extern "C" fn SetShortArrayRegion(
        env: *mut JNIEnv,
        arg2: jshortArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jshort,
    ) {
        todo!("SetShortArrayRegion")
    }

    pub extern "C" fn SetIntArrayRegion(
        env: *mut JNIEnv,
        arg2: jintArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jint,
    ) {
        todo!("SetIntArrayRegion")
    }

    pub extern "C" fn SetLongArrayRegion(
        env: *mut JNIEnv,
        arg2: jlongArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jlong,
    ) {
        todo!("SetLongArrayRegion")
    }

    pub extern "C" fn SetFloatArrayRegion(
        env: *mut JNIEnv,
        arg2: jfloatArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jfloat,
    ) {
        todo!("SetFloatArrayRegion")
    }

    pub extern "C" fn SetDoubleArrayRegion(
        env: *mut JNIEnv,
        arg2: jdoubleArray,
        arg3: jsize,
        arg4: jsize,
        arg5: *const jdouble,
    ) {
        todo!("SetDoubleArrayRegion")
    }

    pub extern "C" fn RegisterNatives(
        env: *mut JNIEnv,
        arg2: jclass,
        arg3: *const JNINativeMethod,
        arg4: jint,
    ) -> jint {
        todo!("RegisterNatives")
    }

    pub extern "C" fn UnregisterNatives(env: *mut JNIEnv, arg2: jclass) -> jint {
        todo!("UnregisterNatives")
    }

    pub extern "C" fn MonitorEnter(env: *mut JNIEnv, arg2: jobject) -> jint {
        todo!("MonitorEnter")
    }

    pub extern "C" fn MonitorExit(env: *mut JNIEnv, arg2: jobject) -> jint {
        todo!("MonitorExit")
    }

    pub extern "C" fn GetJavaVM(env: *mut JNIEnv, arg2: *mut *mut JavaVM) -> jint {
        todo!("GetJavaVM")
    }

    pub extern "C" fn GetStringRegion(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut jchar,
    ) {
        todo!("GetStringRegion")
    }

    pub extern "C" fn GetStringUTFRegion(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: jsize,
        arg4: jsize,
        arg5: *mut ::std::os::raw::c_char,
    ) {
        todo!("GetStringUTFRegion")
    }

    pub extern "C" fn GetPrimitiveArrayCritical(
        env: *mut JNIEnv,
        arg2: jarray,
        arg3: *mut jboolean,
    ) -> *mut ::std::os::raw::c_void {
        todo!("GetPrimitiveArrayCritical")
    }

    pub extern "C" fn ReleasePrimitiveArrayCritical(
        env: *mut JNIEnv,
        arg2: jarray,
        arg3: *mut ::std::os::raw::c_void,
        arg4: jint,
    ) {
        todo!("ReleasePrimitiveArrayCritical")
    }

    pub extern "C" fn GetStringCritical(
        env: *mut JNIEnv,
        arg2: jstring,
        arg3: *mut jboolean,
    ) -> *const jchar {
        todo!("GetStringCritical")
    }

    pub extern "C" fn ReleaseStringCritical(env: *mut JNIEnv, arg2: jstring, arg3: *const jchar) {
        todo!("ReleaseStringCritical")
    }

    pub extern "C" fn NewWeakGlobalRef(env: *mut JNIEnv, arg2: jobject) -> jweak {
        todo!("NewWeakGlobalRef")
    }

    pub extern "C" fn DeleteWeakGlobalRef(env: *mut JNIEnv, arg2: jweak) {
        todo!("DeleteWeakGlobalRef")
    }

    pub extern "C" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
        todo!("ExceptionCheck")
    }

    pub extern "C" fn NewDirectByteBuffer(
        env: *mut JNIEnv,
        arg2: *mut ::std::os::raw::c_void,
        arg3: jlong,
    ) -> jobject {
        todo!("NewDirectByteBuffer")
    }

    pub extern "C" fn GetDirectBufferAddress(
        env: *mut JNIEnv,
        arg2: jobject,
    ) -> *mut ::std::os::raw::c_void {
        todo!("GetDirectBufferAddress")
    }

    pub extern "C" fn GetDirectBufferCapacity(env: *mut JNIEnv, arg2: jobject) -> jlong {
        todo!("GetDirectBufferCapacity")
    }

    pub extern "C" fn GetObjectRefType(env: *mut JNIEnv, arg2: jobject) -> jobjectRefType {
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
