# TODOs (81)
 * [cafebabe/src/class.rs](cafebabe/src/class.rs) (3)
   * `// TODO validate combinations`
   * `// TODO detect dups with same name & descriptor`
   * `// TODO detect dups with same name & descriptor`
 * [cafebabe/src/constant_pool/attribute.rs](cafebabe/src/constant_pool/attribute.rs) (2)
   * `// TODO exception handlers`
   * `// TODO attributes`
 * [cafebabe/src/constant_pool/item.rs](cafebabe/src/constant_pool/item.rs) (5)
   * `// TODO these are Entries not Items`
   * `// TODO handle specific versions tags were introduced`
   * `// TODO float might need extra parsing`
   * `// TODO double might need extra parsing`
   * `// TODO is_loadable()`
 * [cafebabe/src/types.rs](cafebabe/src/types.rs) (4)
   * `// TODO resolve constant pool entries`
   * `// TODO reduce duplication`
   * `// TODO validate combinations`
   * `// TODO validate combinations`
 * [src/alloc.rs](src/alloc.rs) (4)
   * `// TODO gc arena`
   * `// TODO actually intern strings`
   * `// TODO methods on VmRef newtype`
   * `// TODO oom error`
 * [src/class.rs](src/class.rs) (21)
   * `// TODO store dimensions`
   * `/// TODO weak reference for cyclic?`
   * `// TODO arrays should live on the GC java heap`
   * `// TODO arrays should be specialised and not hold massive DataValues`
   * `// TODO arrayvec`
   * `// TODO get classloader reference from tls instead of parameter`
   * `// TODO verify constant pool offsets so we can raise a single classformaterror then trust it`
   * `// TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2`
   * `// TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors`
   * `// TODO Every array type implements the interfaces Cloneable and java.io.Serializable.`
   * `// update ptr - TODO use Arc::get_unchecked_mut when it is stable`
   * `// TODO set obj->vmdata field to vm_class`
   * `// TODO version to look in (super)interfaces too`
   * `// TODO initialise final static fields from ConstantValue attrs`
   * `// TODO wrap exception here and return the proper type`
   * `// TODO proper exception type here`
   * `// TODO specific exception type e.g. ExceptionInInitializerError`
   * `// TODO just allocate an object instead of this unsafeness`
   * `// TODO inherit superclass fields too`
   * `// TODO limit array length to i32::MAX somewhere`
   * `// TODO not quite correct toString`
 * [src/classloader.rs](src/classloader.rs) (11)
   * `// TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;`
   * `// TODO register class "package" with loader (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)`
   * `// TODO actually instantiate exceptions`
   * `// TODO run user classloader first`
   * `// TODO array classes are treated differently`
   * `// TODO wait for other thread to finish loading`
   * `// TODO record that this loader is an initiating loader`
   * `let bytes = std::fs::read(path).expect("io error"); // TODO java.lang.IOError`
   * `// TODO define hardcoded preload classes in a better way`
   * `// TODO add array lookup with enum constants for common symbols like Object, or perfect hashing`
   * `// TODO newtype VmRef should handle equality`
 * [src/classpath.rs](src/classpath.rs) (1)
   * `// TODO enum for path type, zip/jar or directory`
 * [src/constant_pool.rs](src/constant_pool.rs) (2)
   * `// TODO store interned string instance here`
   * `// TODO A numeric constant of type long or double OR A symbolic reference to a`
 * [src/error.rs](src/error.rs) (3)
   * `// TODO reference to class instead of name`
   * `// TODO reference to cause`
   * `// TODO backtrace`
 * [src/interpreter/frame.rs](src/interpreter/frame.rs) (6)
   * `// TODO validate local var slot in case of wide vars`
   * `// TODO longs and doubles take 2 slots!`
   * `// TODO tests for operand stack and local var array`
   * `// TODO instead of options, enum {Instance(obj), Static(class)}`
   * `// TODO pass args to native function`
   * `// TODO impl Display for mstr`
 * [src/interpreter/insn/instruction.rs](src/interpreter/insn/instruction.rs) (9)
   * `// TODO better handling of interpreter error`
   * `// TODO some 2s are signed`
   * `insn_2x!(Iinc, "iinc"); // TODO second is signed byte, or just store separate u8s`
   * `// TODO ensure class is not interface, method not abstract, not constructor`
   * `// TODO typecheck args at verification time`
   * `// TODO lookup natively interned string instance`
   * `// TODO natively intern new string instance`
   * `} // TODO int/float`
   * `// TODO class symbolic reference`
 * [src/jvm.rs](src/jvm.rs) (4)
   * `// TODO "catch" any exception during init, and log it properly with stacktrace etc`
   * `// TODO set all properties in gnu/classpath/VMSystemProperties.preinit`
   * `// TODO wait for threads to die, unintialise TLS, assert this is the last ref to global state`
   * `// TODO actually parse args with something like clap`
 * [src/properties.rs](src/properties.rs) (2)
   * `// TODO remaining static ones`
   * `// TODO dynamic ones e.g. user.home`
 * [src/thread.rs](src/thread.rs) (1)
   * `exception: RefCell<Option<VmRef<Throwable /* TODO vmobject */>>>,`
 * [src/types.rs](src/types.rs) (3)
   * `// TODO more efficient packing of data types, dont want huge enum discriminant taking up all the space`
   * `// TODO interned strings for class names?`
   * `// TODO gross that we always need an allocation for reference type - Cow for str and store array dim inline?`
