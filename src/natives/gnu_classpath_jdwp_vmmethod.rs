use crate::alloc::VmRef;
use crate::class::FunctionArgs;
use crate::error::Throwable;
use crate::types::DataValue;

/// ()Ljava/lang/String;
pub fn get_name(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmmethod::get_name")
}

/// ()Ljava/lang/String;
pub fn get_signature(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmmethod::get_signature")
}

/// ()I
pub fn get_modifiers(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmmethod::get_modifiers")
}

/// ()Lgnu/classpath/jdwp/util/LineTable;
pub fn get_line_table(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmmethod::get_line_table")
}

/// ()Lgnu/classpath/jdwp/util/VariableTable;
pub fn get_variable_table(_: FunctionArgs) -> Result<Option<DataValue>, VmRef<Throwable>> {
    todo!("native method gnu_classpath_jdwp_vmmethod::get_variable_table")
}
