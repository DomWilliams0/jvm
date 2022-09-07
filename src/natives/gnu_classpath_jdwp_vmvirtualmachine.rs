use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// (Ljava/lang/Thread;)V
pub fn suspend_thread(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::suspend_thread")
}

/// (Ljava/lang/Thread;)V
pub fn resume_thread(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::resume_thread")
}

/// (Ljava/lang/Thread;)I
pub fn get_suspend_count(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_suspend_count")
}

/// ()Ljava/util/Collection;
pub fn get_all_loaded_classes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_all_loaded_classes")
}

/// (Ljava/lang/Class;)I
pub fn get_class_status(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_class_status")
}

/// (Ljava/lang/Class;)[Lgnu/classpath/jdwp/VMMethod;
pub fn get_all_class_methods(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_all_class_methods")
}

/// (Ljava/lang/Class;J)Lgnu/classpath/jdwp/VMMethod;
pub fn get_class_method(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_class_method")
}

/// (Ljava/lang/Thread;II)Ljava/util/ArrayList;
pub fn get_frames(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_frames")
}

/// (Ljava/lang/Thread;J)Lgnu/classpath/jdwp/VMFrame;
pub fn get_frame(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_frame")
}

/// (Ljava/lang/Thread;)I
pub fn get_frame_count(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_frame_count")
}

/// (Ljava/lang/Thread;)I
pub fn get_thread_status(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_thread_status")
}

/// (Ljava/lang/ClassLoader;)Ljava/util/ArrayList;
pub fn get_load_requests(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_load_requests")
}

/// (Ljava/lang/Object;Ljava/lang/Thread;Ljava/lang/Class;Lgnu/classpath/jdwp/VMMethod;[Lgnu/classpath/jdwp/value/Value;I)Lgnu/classpath/jdwp/util/MethodResult;
pub fn execute_method(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::execute_method")
}

/// (Ljava/lang/Class;)Ljava/lang/String;
pub fn get_source_file(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_source_file")
}

/// (Lgnu/classpath/jdwp/event/EventRequest;)V
pub fn register_event(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::register_event")
}

/// (Lgnu/classpath/jdwp/event/EventRequest;)V
pub fn unregister_event(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::unregister_event")
}

/// (B)V
pub fn clear_events(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::clear_events")
}

/// ([Ljava/lang/Class;[[B)V
pub fn redefine_classes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::redefine_classes")
}

/// (Ljava/lang/String;)V
pub fn set_default_stratum(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::set_default_stratum")
}

/// (Ljava/lang/Class;)Ljava/lang/String;
pub fn get_source_debug_extension(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_source_debug_extension")
}

/// (Lgnu/classpath/jdwp/VMMethod;)[B
pub fn get_bytecodes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_bytecodes")
}

/// (Ljava/lang/Object;)Lgnu/classpath/jdwp/util/MonitorInfo;
pub fn get_monitor_info(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_monitor_info")
}

/// (Ljava/lang/Thread;)[Ljava/lang/Object;
pub fn get_owned_monitors(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_owned_monitors")
}

/// (Ljava/lang/Thread;)Ljava/lang/Object;
pub fn get_current_contended_monitor(
    _: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_current_contended_monitor")
}

/// (Ljava/lang/Thread;J)V
pub fn pop_frames(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::pop_frames")
}
