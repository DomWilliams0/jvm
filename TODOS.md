# TODOs (76)
 * [cafebabe/src/class.rs](cafebabe/src/class.rs) (3)
   * `// TODO validate combinations`
   * `// TODO detect dups with same name & descriptor`
   * `// TODO detect dups with same name & descriptor`
 * [cafebabe/src/constant_pool/attribute.rs](cafebabe/src/constant_pool/attribute.rs) (2)
   * `// TODO exception handlers`
   * `// TODO attributes`
 * [cafebabe/src/constant_pool/item.rs](cafebabe/src/constant_pool/item.rs) (4)
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
 * [src/class.rs](src/class.rs) (13)
   * `/// TODO weak reference for cyclic?`
   * `// TODO verify constant pool offsets so we can raise a single classformaterror then trust it`
   * `// TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2`
   * `// TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors`
   * `// TODO Every array type implements the interfaces Cloneable and java.io.Serializable.`
   * `// update ptr - TODO use Arc::get_unchecked_mut when it is stable`
   * `// TODO set obj->vmdata field to vm_class`
   * `// TODO initialise final static fields from ConstantValue attrs`
   * `return Err(Throwables::ClassFormatError); // TODO different exception`
   * `// TODO specific exception type e.g. ExceptionInInitializerError`
   * `// TODO just allocate an object instead of this unsafeness`
   * `// TODO inherit superclass fields too`
   * `// TODO not quite correct toString`
 * [src/classloader.rs](src/classloader.rs) (11)
   * `// TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;`
   * `// TODO register class "package" with loader (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)`
   * `// TODO actually instantiate exceptions`
   * `// TODO run user classloader first`
   * `// TODO array classes are treated differently`
   * `// TODO wait for other thread to finish loading`
   * `// TODO record that this loader is an initiating loader`
   * `// TODO array class access = element class access or fully accessible`
   * `let bytes = std::fs::read(path).expect("io error"); // TODO java.lang.IOError`
   * `// TODO add array lookup with enum constants for common symbols like Object, or perfect hashing`
   * `// TODO newtype VmRef should handle equality`
 * [src/classpath.rs](src/classpath.rs) (1)
   * `// TODO enum for path type, zip/jar or directory`
 * [src/constant_pool.rs](src/constant_pool.rs) (1)
   * `// TODO A numeric constant of type long or double OR A symbolic reference to a`
 * [src/error.rs](src/error.rs) (3)
   * `// TODO reference to class instead of name`
   * `// TODO reference to cause`
   * `// TODO backtrace`
 * [src/interpreter/frame.rs](src/interpreter/frame.rs) (3)
   * `// TODO validate local var slot in case of wide vars`
   * `// TODO tests for operand stack and local var array`
   * `// TODO instead of options, enum {Instance(obj), Static(class)}`
 * [src/interpreter/insn/bytecode.rs](src/interpreter/insn/bytecode.rs) (2)
   * `// TODO temporary, dont log every single instruction`
   * `// TODO verified version of Bytecode that doesn't do all the safety checks for speed e.g. fn parse_unverified(bytes) -> Self`
 * [src/interpreter/insn/instruction.rs](src/interpreter/insn/instruction.rs) (7)
   * `// TODO n variations`
   * `// TODO intern string`
   * `// TODO call string constructor`
   * `// TODO char array`
   * `} // TODO int/float`
   * `// TODO class symbolic reference`
   * `// TODO instantiate string`
 * [src/interpreter/interp.rs](src/interpreter/interp.rs) (7)
   * `// TODO get current class`
   * `// TODO get current method`
   * `// TODO get current frame`
   * `// TODO native frames`
   * `// TODO verify, "compile" and cache instructions`
   * `// TODO abrupt exit with proper exception creation`
   * `// TODO handle return`
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
 * [src/types.rs](src/types.rs) (4)
   * `// TODO more efficient packing of data types, dont want huge enum discriminant taking up all the space`
   * `// TODO interned strings for class names?`
   * `// TODO MString method from owned utf8 to avoid this copy`
   * `// TODO avoid extra allocation here too`
