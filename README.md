# jvm

Toy JVM that might be useful. Uses GNU Classpath.

## Progress

* [o] [Implement all opcodes](src/interpreter/insn/instruction.rs)
  * [ ] Implement them *correctly*
* [o] [Implement enough native methods](src/bootstrap.rs) to bootstrap the standard library
  * [.] Implement enough of the JNI too
* [ ] Execute a simple `System.out.println" call
* [ ] Integration test rig to compare output to a reference implementation
  * [ ] Fix all the bugs and unimplemented opcodes
* [ ] Fast, non-verifying interpreter optimised for speed

## Usage

*Not recommended in its current state*

A nightly rust compiler is required for variadic functions in the JNI interface.

* Download, build and install [GNU Classpath](https://www.gnu.org/software/classpath/faq/faq.html#faq3_5)
    * I had luck with the following commands: `CFLAGS="-w" ./configure --enable-load-library=yes --with-glibj=flat --enable-jni=yes --disable-gjdoc && make && make install`
    * I also had to install `antlr` 3.5.2 and downgrade to a JDK that still has `javah` (OpenJDK 8)
* Find/compile a **simple** `.java` file into a `.class`
* `cargo run -- <class name> --Xbootclasspath <colon separated list of paths to system classes> --cp <colon separated list of paths for non-system classes>`
    * Example: `cargo run -- com.me.MyClass --Xbootclasspath /gnuclasspath:../java --cp ../java`. This will (try to) run the main method of `../java/com/me/MyClass.class`
    * See help menu for more: `cargo run -- --help`
