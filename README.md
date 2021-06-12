# jvm

Toy JVM that might be feature complete one day. Uses GNU Classpath.

## Progress

* [X] JVM skeleton to "execute" a class with an empty main
* [ ] [Implement enough native methods](src/bootstrap.rs) to bootstrap the standard library
* [ ] Execute a simple `System.out.println" call
* [ ] Integration test rig to compare output to a reference implementation
  * [ ] Fix all the bugs and unimplemented opcodes

## Usage

*Not recommended in its current state*

* Download, build and install [GNU Classpath](https://www.gnu.org/software/classpath/faq/faq.html#faq3_5)
    * I had luck with the following commands: `CFLAGS="-w" ./configure --enable-load-library=yes --with-glibj=flat --enable-jni=yes --disable-gjdoc && make && make install`
    * I also had to install `antlr` 3.5.2 and downgrade to a JDK that still has `javah` (OpenJDK 8)
* Find/compile a **simple** `.java` file into a `.class`
* `cargo run -- <class name> --Xbootclasspath <colon separated list of paths to system classes i.e. GNU classpath> --cp <colon separated list of paths for non-system classes>`
    * Example: `cargo run -- com.me.MyClass --Xbootclasspath /gnuclasspath:../java --cp ../java`. This will (try to) run the main method of `../java/com/me/MyClass.class`
