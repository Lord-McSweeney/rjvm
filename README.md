# Java Virtual Machine implementation in Rust

`rjvm` is a fast JVM that supports a fairly high number of Java features while
still remaining portable. It can be compiled on any platform that supports
Rust and has `alloc` support.

`rjvm` is also very modular. The core JVM, `rjvm_core`, is `no_std` and requires
only a few standard classes to be provided during its bootstrap process. The
base globals implementations, `rjvm_globals`, is also `no_std` and remains under
1 MB. Users such as `rjvm_web` and `rjvm_desktop` provide their own
implementations for platform-dependent behavior, such as loading classes from
the filesystem, I/O, and reading date and time.

Both `rjvm_core` and to a lesser extent `rjvm_globals` try to have as few direct
and indirect dependencies as possible.

## Features
- bytecode verifier
- not-painfully-slow interpreter
- All language features before Java 7 except for threads
- I/O operations, including file I/O (only for native platforms)
- support for many classes in the `java.io`, `java.lang`, `java.lang.reflect`, and `java.util` packages
- some regex support
- implementations for most `ClassLoader` and reflection methods
- ability to run Java on the browser through WASM
- `no_std` support
    - `alloc` is still required

## Upcoming features
- Proper encoding/decoding
- Implementations for the remaining stack operations
- Windows support
- Array type verification in verifier
- Object class verification in verifier
- Better performance
    - 8-byte Value
    - split fields into object and primitive fields?
    - other?

## Missing features
- Proper charset support
- `invokedynamic` (Java 8 lambdas) support
- Multithreading (`Thread.start`) support
- Lots of API
- Better garbage collection
- A JIT compiler
