# TODOs (119)
 * [cafebabe/src/class.rs](cafebabe/src/class.rs) (3)
   * `// TODO validate combinations`
   * `// TODO detect dups with same name & descriptor`
   * `// TODO detect dups with same name & descriptor`
 * [cafebabe/src/constant_pool/attribute.rs](cafebabe/src/constant_pool/attribute.rs) (2)
   * `// TODO exception handlers`
   * `// TODO attributes`
 * [cafebabe/src/constant_pool/entry.rs](cafebabe/src/constant_pool/entry.rs) (1)
   * `// TODO parse desc to DataType here in FieldRefEntry`
 * [cafebabe/src/constant_pool/item.rs](cafebabe/src/constant_pool/item.rs) (4)
   * `// TODO handle specific versions tags were introduced`
   * `// TODO float might need extra parsing`
   * `// TODO double might need extra parsing`
   * `// TODO is_loadable()`
 * [cafebabe/src/types.rs](cafebabe/src/types.rs) (5)
   * `// TODO resolve constant pool entries`
   * `// TODO reduce duplication`
   * `// TODO validate combinations`
   * `// TODO validate combinations`
   * `// TODO dont truncate to make this cheaper`
 * [src/alloc.rs](src/alloc.rs) (4)
   * `// TODO gc arena`
   * `// TODO actually intern strings`
   * `// TODO methods on VmRef newtype`
   * `// TODO oom error`
 * [src/class.rs](src/class.rs) (26)
   * `// TODO store dimensions`
   * `/// TODO weak reference for cyclic?`
   * `// TODO arrays should live on the GC java heap`
   * `// TODO arrays should be specialised and not hold massive DataValues`
   * `// TODO arrayvec`
   * `// TODO get classloader reference from tls instead of parameter`
   * `// TODO verify constant pool offsets so we can raise a single classformaterror then trust it`
   * `// TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2`
   * `// TODO precalculate capacity`
   * `// TODO no need to iterate interfaces when looking for instance fields, add separate iterator method`
   * `// TODO are static fields treated and resolved the same as instance fields?`
   * `// TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors`
   * `// TODO Every array type implements the interfaces Cloneable and java.io.Serializable.`
   * `// update ptr - TODO use Arc::get_unchecked_mut when it is stable`
   * `// TODO set obj->vmdata field to vm_class`
   * `// TODO version to look in (super)interfaces too`
   * `// TODO ensure there is only 1`
   * `// TODO initialise final static fields from ConstantValue attrs`
   * `// TODO only do this if its a class and not an iface`
   * `// TODO wrap exception here and return the proper type`
   * `// TODO proper exception type here`
   * `// TODO specific exception type e.g. ExceptionInInitializerError`
   * `// TODO method.as_full_name() impls Debug to make this easier`
   * `// TODO just allocate an object instead of this unsafeness`
   * `// TODO limit array length to i32::MAX somewhere`
   * `// TODO not quite correct toString`
 * [src/classloader.rs](src/classloader.rs) (15)
   * `// TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;`
   * `// TODO register class "package" with loader (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)`
   * `// TODO actually instantiate exceptions`
   * `/// TODO use a FnOnce() -> WhichLoader or &WhichLoader to avoid many useless clones`
   * `// TODO run user classloader first`
   * `// TODO array classes are treated differently`
   * `// TODO wait for other thread to finish loading`
   * `// TODO record that this loader is an initiating loader`
   * `let bytes = std::fs::read(path).expect("io error"); // TODO java.lang.IOError`
   * `// TODO define hardcoded preload classes in a better way`
   * `// TODO add array lookup with enum constants for common symbols like Object, or perfect hashing`
   * `// TODO mstr display impl`
   * `// TODO calling a method from native needs to be more ergonomic`
   * `// TODO interpreter error -> internal vm error throwable`
   * `// TODO newtype VmRef should handle equality`
 * [src/classpath.rs](src/classpath.rs) (1)
   * `// TODO enum for path type, zip/jar or directory`
 * [src/constant_pool.rs](src/constant_pool.rs) (3)
   * `// TODO store interned string instance here`
   * `// TODO method and field refs should be resolved vtable indices instead of loads of strings`
   * `// TODO A numeric constant of type long or double OR A symbolic reference to a`
 * [src/error.rs](src/error.rs) (3)
   * `// TODO reference to class instead of name`
   * `// TODO reference to cause`
   * `// TODO backtrace`
 * [src/interpreter/error.rs](src/interpreter/error.rs) (1)
   * `// TODO combine repetetive errors for different data types`
 * [src/interpreter/frame.rs](src/interpreter/frame.rs) (7)
   * `// TODO validate local var slot in case of wide vars`
   * `// TODO longs and doubles take 2 slots!`
   * `// TODO tests for operand stack and local var array`
   * `// TODO impl display on mstr`
   * `// TODO expects()`
   * `// TODO impl Display for mstr`
   * `// TODO move these to extension trait on operandstack`
 * [src/interpreter/insn/instruction.rs](src/interpreter/insn/instruction.rs) (28)
   * `// TODO operand stack pop then verify might be wrong - only pop if its the right type?`
   * `/// TODO might be possible to continue with resolved methods/fields state instead of replay`
   * `// TODO better handling of interpreter error`
   * `// TODO some 2s are signed`
   * `// TODO catch this at verification time`
   * `// TODO sign extended?`
   * `// TODO narrow float to int properly`
   * `// TODO is probably wrong`
   * `// TODO "converted to the float result using IEEE 754 round to nearest mode"`
   * `// TODO return error here`
   * `// TODO ensure method is not static, IncompatibleClassChangeError`
   * `// TODO native method`
   * `// TODO ensure class is not interface, method not abstract, not constructor`
   * `// TODO typecheck args at verification time`
   * `// TODO lookup natively interned string instance`
   * `// TODO natively intern new string instance`
   * `// TODO int constant`
   * `// TODO deny long and double`
   * `// TODO class symbolic reference`
   * `// TODO ensure not abstract, throw InstantiationError`
   * `// TODO verify not array class`
   * `// TODO throw IncompatibleClassChangeError`
   * `// TODO check value is compatible with field desc`
   * `// TODO if final can only be in constructor`
   * `// TODO throw IncompatibleClassChangeError`
   * `// TODO check value is compatible with field desc`
   * `// TODO if final can only be in constructor`
   * `// TODO if class is interface then can only be in constructor`
 * [src/interpreter/interp.rs](src/interpreter/interp.rs) (1)
   * `// TODO pass these into execute()`
 * [src/jvm.rs](src/jvm.rs) (5)
   * `// TODO "catch" any exception during init, and log it properly with stacktrace etc`
   * `// TODO set all properties in gnu/classpath/VMSystemProperties.preinit`
   * `// TODO populate String[] args`
   * `// TODO wait for threads to die, unintialise TLS, assert this is the last ref to global state`
   * `// TODO standard jvm args`
 * [src/properties.rs](src/properties.rs) (2)
   * `// TODO remaining static ones`
   * `// TODO dynamic ones e.g. user.home`
 * [src/storage.rs](src/storage.rs) (5)
   * `// TODO field storage should be inline in VmRef<Object>`
   * `// TODO compact field storage i.e. not using DataValue enum`
   * `// TODO phantom generic type to tag this as Static or Instance fields`
   * `#[derive(Debug)] // TODO fieldstorage better debug impl`
   * `// TODO test this once structure is settled`
 * [src/thread.rs](src/thread.rs) (1)
   * `exception: RefCell<Option<VmRef<Throwable /* TODO vmobject */>>>,`
 * [src/types.rs](src/types.rs) (2)
   * `// TODO more efficient packing of data values`
   * `// TODO narrowing`
