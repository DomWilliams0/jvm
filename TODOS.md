# TODOs (45)
 * [src/alloc.rs](src/alloc.rs) (3)
   * `// TODO gc arena`
   * `// TODO actually intern strings`
   * `// TODO methods on VmRef newtype`
 * [src/class.rs](src/class.rs) (7)
   * `/// TODO weak reference for cyclic?`
   * `// TODO just allocate an object instead of this unsafeness`
   * `// TODO verify constant pool offsets so we can raise a single classformaterror then trust it`
   * `// TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2`
   * `// TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors`
   * `// TODO set obj->vmdata field to vm_class`
   * `// TODO not quite correct toString`
 * [src/classloader.rs](src/classloader.rs) (9)
   * `// TODO linked?`
   * `// TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;`
   * `// TODO register class "package" with loader (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)`
   * `// TODO actually instantiate exceptions`
   * `return Err(Throwables::ClassFormatError); // TODO different exception`
   * `// TODO ClassLoaderRef that holds an upgradable rwlock guard, so no need to hold the lock for the whole method`
   * `// TODO actually update and use load state`
   * `// TODO array classes are treated differently`
   * `let bytes = std::fs::read(path).expect("io error"); // TODO java.lang.IOError`
 * [src/classpath.rs](src/classpath.rs) (1)
   * `// TODO enum for path type, zip/jar or directory`
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
 * [src/interpreter/insn/instruction.rs](src/interpreter/insn/instruction.rs) (1)
   * `// TODO n variations`
 * [src/interpreter/interp.rs](src/interpreter/interp.rs) (5)
   * `// TODO get current class`
   * `// TODO get current method`
   * `// TODO get current frame`
   * `// TODO native frames`
   * `// TODO verify, "compile" and cache instructions`
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
