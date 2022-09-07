use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

pub fn suspend_thread(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::suspend_thread")
}

pub fn resume_thread(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::resume_thread")
}

pub fn get_suspend_count(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_suspend_count")
}

pub fn get_all_loaded_classes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_all_loaded_classes")
}

pub fn get_class_status(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_class_status")
}

pub fn get_all_class_methods(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_all_class_methods")
}

pub fn get_class_method(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_class_method")
}

pub fn get_frames(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_frames")
}

pub fn get_frame(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_frame")
}

pub fn get_frame_count(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_frame_count")
}

pub fn get_thread_status(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_thread_status")
}

pub fn get_load_requests(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_load_requests")
}

pub fn execute_method(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::execute_method")
}

pub fn get_source_file(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_source_file")
}

pub fn register_event(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::register_event")
}

pub fn unregister_event(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::unregister_event")
}

pub fn clear_events(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::clear_events")
}

pub fn redefine_classes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::redefine_classes")
}

pub fn set_default_stratum(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::set_default_stratum")
}

pub fn get_source_debug_extension(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_source_debug_extension")
}

pub fn get_bytecodes(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_bytecodes")
}

pub fn get_monitor_info(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_monitor_info")
}

pub fn get_owned_monitors(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_owned_monitors")
}

pub fn get_current_contended_monitor(
    _: FunctionArgs,
) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::get_current_contended_monitor")
}

pub fn pop_frames(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmvirtualmachine::pop_frames")
}
