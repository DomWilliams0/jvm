# TODOs (182)
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
 * [src/class/class.rs](src/class/class.rs) (24)
   * `// TODO when a ClassLoader is dropped, ensure all native libraries associated with it are freed too`
   * `/// TODO weak reference for cyclic reference?`
   * `// TODO store dimensions`
   * `// TODO arrayvec`
   * `// TODO get classloader reference from tls instead of parameter`
   * `// TODO this crashes in release builds, oops`
   * `// TODO verify constant pool offsets so we can raise a single classformaterror then trust it`
   * `// TODO preparation? https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.2`
   * `// TODO different order for field layout than resolution? supers first to enable slicing? or not needed?`
   * `// TODO no need to iterate interfaces when looking for instance fields, add separate iterator method`
   * `// TODO do verification first to throw ClassFormatErrors, then this should not throw any classformaterrors`
   * `// TODO Every array type implements the interfaces Cloneable and java.io.Serializable.`
   * `// TODO search in super classes too?`
   * `// TODO ensure there is only 1`
   * `// TODO also this check, wtf does it mean:`
   * `// TODO this is basically copied from getstatic, reuse instruction impl if possible`
   * `// TODO use Arc::get_mut_unchecked instead when stable`
   * `// TODO initialise final static fields from ConstantValue attrs`
   * `// TODO wrap exception here and return the proper type`
   * `// TODO proper exception type here`
   * `// TODO specific exception type e.g. ExceptionInInitializerError`
   * `// TODO cache mangled name in the method`
   * `// TODO this is basically copied from getstatic, reuse instruction impl if possible`
   * `// TODO compile java source code at test time`
 * [src/class/loader.rs](src/class/loader.rs) (13)
   * `// TODO types for str to differentiate java/lang/Object, java.lang.Object and descrptors e.g. Ljava/lang/Object;`
   * `// TODO register class "package" with loader (https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.3)`
   * `// TODO actually instantiate exceptions`
   * `/// TODO use a FnOnce() -> WhichLoader or &WhichLoader to avoid many useless clones`
   * `// TODO run user classloader first`
   * `// TODO array classes are treated differently`
   * `// TODO wait for other thread to finish loading`
   * `// TODO record that this loader is an initiating loader`
   * `// TODO get thread interpreter and current class automatically`
   * `Err(FindClassError::Io(err)) => panic!("io error: {}", err), // TODO java.lang.IOError`
   * `// TODO add array lookup with enum constants for common symbols like Object, or perfect hashing`
   * `// TODO cache this`
   * `// TODO newtype VmRef should handle equality`
 * [src/class/object.rs](src/class/object.rs) (7)
   * `// TODO arrays should live on the GC java heap`
   * `// TODO arrays should be specialised and not hold massive DataValues`
   * `// TODO mutex only needed in edge case, try with atomic op first`
   * `// TODO just allocate an object instead of this unsafeness`
   * `// TODO limit array length to i32::MAX somewhere`
   * `// TODO do this without all the allocations`
   * `// TODO not quite correct toString`
 * [src/classpath.rs](src/classpath.rs) (3)
   * `// TODO enum for path type, zip/jar or directory`
   * `// TODO awful, fix this`
   * `// TODO fix in miri`
 * [src/constant_pool.rs](src/constant_pool.rs) (3)
   * `// TODO store interned string instance here`
   * `// TODO method and field refs should be resolved vtable indices instead of loads of strings`
   * `// TODO OR A symbolic reference to a dynamically-computed constant whose field descriptor is J (denoting long) or D (denoting double)`
 * [src/debug.rs](src/debug.rs) (1)
   * `// TODO log IO error`
 * [src/error.rs](src/error.rs) (3)
   * `// TODO reference to class instead of name`
   * `// TODO reference to cause`
   * `// TODO backtrace`
 * [src/exec_helper.rs](src/exec_helper.rs) (2)
   * `// TODO class arg should be a trait for either class name &str or class reference`
   * `.unwrap(); // TODO handle exc`
 * [src/interpreter/error.rs](src/interpreter/error.rs) (1)
   * `// TODO combine repetetive errors for different data types`
 * [src/interpreter/frame.rs](src/interpreter/frame.rs) (8)
   * `// TODO is this always the same as method.class() ?`
   * `// TODO validate local var slot in case of wide vars`
   * `// TODO longs and doubles take 2 slots!`
   * `// TODO tests for operand stack and local var array`
   * `// TODO expects()`
   * `// TODO long and double are wide`
   * `// TODO generic helper methods for popping up to 3 types from stack`
   * `// TODO move these to extension trait on operandstack`
 * [src/interpreter/insn/instruction.rs](src/interpreter/insn/instruction.rs) (39)
   * `// TODO operand stack pop then verify might be wrong - only pop if its the right type?`
   * `/// TODO might be possible to continue with resolved methods/fields state instead of replay`
   * `// TODO better handling of interpreter error`
   * `// TODO some 2s are signed`
   * `// TODO sign extended?`
   * `// TODO check value type, throw if bad`
   * `// TODO actually bounds check`
   * `// TODO assignment compatibility check`
   * `// TODO value set conversion`
   * `// TODO narrow float to int properly`
   * `// TODO is probably wrong`
   * `// TODO value set conversion`
   * `// TODO "converted to the float result using IEEE 754 round to nearest mode"`
   * `// TODO invokeinterface throws a lot more exceptions`
   * `// TODO NoSuchMethod error`
   * `// TODO ensure method is not static, IncompatibleClassChangeError`
   * `// TODO verify this`
   * `// TODO ensure not abstract`
   * `// TODO return error here`
   * `// TODO ensure method is not static, IncompatibleClassChangeError`
   * `// TODO native method`
   * `// TODO ensure class is not interface, method not abstract, not constructor`
   * `// TODO typecheck args at verification time`
   * `// TODO ensure method is not static, IncompatibleClassChangeError`
   * `// TODO may need to convert int to byte/short etc first`
   * `// TODO lookup natively interned string instance`
   * `// TODO natively intern new string instance`
   * `// TODO deny long and double`
   * `// TODO monitorenter`
   * `// TODO monitorexit`
   * `// TODO ensure not abstract, throw InstantiationError`
   * `// TODO verify not array class`
   * `// TODO throw IncompatibleClassChangeError`
   * `// TODO check value is compatible with field desc`
   * `// TODO if final can only be in constructor`
   * `// TODO throw IncompatibleClassChangeError`
   * `// TODO check value is compatible with field desc`
   * `// TODO if final can only be in constructor`
   * `// TODO if class is interface then can only be in constructor`
 * [src/interpreter/interp.rs](src/interpreter/interp.rs) (3)
   * `// TODO catch this at verification time`
   * `// TODO pass these into execute()`
   * `// TODO better handling of interpreter error`
 * [src/interpreter/native.rs](src/interpreter/native.rs) (4)
   * `// TODO support freeing of thunks when a class is unloaded, and reuse it`
   * `// TODO size depends on number of args to pass`
   * `// TODO float registers`
   * `let register = int_registers.next().expect("TODO: stack spillover");`
 * [src/jit/mod.rs](src/jit/mod.rs) (4)
   * `// TODO reorganise into modules`
   * `// TODO actually compile`
   * `// TODO return result`
   * `CompileState::NotCompiled => unreachable!("not queued"), // TODO queue here?`
 * [src/jni/api.rs](src/jni/api.rs) (8)
   * `// TODO get actual env for current thread, rather than a global`
   * `// TODO set exception`
   * `// TODO keep track of global references in jvm or is it ok to leak them like this?`
   * `// TODO throw exception instead of panic`
   * `// TODO throw exception instead of panic`
   * `// TODO throw exception instead of panic`
   * `// TODO this is gross`
   * `// TODO store bytes in string directly?`
 * [src/jni/library.rs](src/jni/library.rs) (2)
   * `// TODO call JNI_OnUnload in Drop`
   * `// TODO does this work on windows too?`
 * [src/jvm.rs](src/jvm.rs) (6)
   * `// TODO "catch" any exception during init, and log it properly with stacktrace etc`
   * `// TODO static initializer is not run?`
   * `// TODO populate String[] args`
   * `// TODO wait for threads to die, unintialise TLS, assert this is the last ref to global state`
   * `// TODO standard jvm args`
   * `// TODO generic -D arg collection`
 * [src/natives/gnu_classpath_vmstackwalker.rs](src/natives/gnu_classpath_vmstackwalker.rs) (1)
   * `// TODO native impls for other VMStackWalker methods`
 * [src/natives/gnu_classpath_vmsystemproperties.rs](src/natives/gnu_classpath_vmsystemproperties.rs) (1)
   * `// TODO actually do preInit`
 * [src/natives/java_lang_class.rs](src/natives/java_lang_class.rs) (2)
   * `// TODO actually register natives`
   * `// TODO get actual assertion status`
 * [src/natives/java_lang_double.rs](src/natives/java_lang_double.rs) (2)
   * `// TODO this is definitely wrong`
   * `// TODO this is definitely wrong`
 * [src/natives/java_lang_float.rs](src/natives/java_lang_float.rs) (1)
   * `// TODO this is definitely wrong`
 * [src/natives/java_lang_vmclass.rs](src/natives/java_lang_vmclass.rs) (2)
   * `// TODO put this into helper`
   * `// TODO pass in cause for loading`
 * [src/natives/java_lang_vmruntime.rs](src/natives/java_lang_vmruntime.rs) (2)
   * `// TODO borrow version of classpath`
   * `// TODO non utf8 paths?`
 * [src/natives/java_lang_vmsystem.rs](src/natives/java_lang_vmsystem.rs) (2)
   * `// TODO check elements really are assignable`
   * `// TODO remove bounds check here, we just checked it explicitly`
 * [src/natives/java_lang_vmthread.rs](src/natives/java_lang_vmthread.rs) (1)
   * `// TODO volatile field!`
 * [src/natives/java_lang_vmthrowable.rs](src/natives/java_lang_vmthrowable.rs) (1)
   * `// TODO implement fillInStackTrace`
 * [src/properties.rs](src/properties.rs) (5)
   * `// TODO these properties are not all correct`
   * `prop!("java.home", java_home.into()); // TODO`
   * `prop!("java.specification.version", "TODO"); // TODO get from Configuration class?`
   * `prop!("java.library.path", library_path.into()); // TODO`
   * `prop!("java.ext.dirs", "."); // TODO`
 * [src/storage.rs](src/storage.rs) (5)
   * `// TODO field storage should be inline in VmRef<Object>`
   * `// TODO compact field storage i.e. not using DataValue enum`
   * `// TODO phantom generic type to tag this as Static or Instance fields`
   * `#[derive(Debug)] // TODO fieldstorage better debug impl`
   * `// TODO test this once structure is settled`
 * [src/thread.rs](src/thread.rs) (3)
   * `exception: RefCell<Option<VmRef<Throwable /* TODO vmobject */>>>,`
   * `return_value: RefCell<Option<DataValue>>, // TODO really needed?`
   * `// set_field("vmdata", DataValue::Reference(todo!()))?; // TODO vmthread struct`
 * [src/types.rs](src/types.rs) (4)
   * `// TODO more efficient packing of data values`
   * `// TODO does boolean conversions count as widening`
   * `// TODO is int->bool technically narrowing and should it be included here?`
   * `// TODO actually check values of converted primitives`
