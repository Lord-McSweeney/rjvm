# Java Virtual Machine implementation in Rust

## Features
- bytecode verifier
- All language features before Java 7 except for threads
- I/O operations, including file I/O
- support for some classes in the `java.io`, `java.lang`, and `java.util` packages
- ability to run Java on the browser through WASM

## Upcoming features
- Implementations for the remaining stack operations
- More `ClassLoader` features
- Windows support
- Array type verification in verifier
- Object class verification in verifier
- Better performance
    - 8-byte Value
    - don't allocate on call
    - split fields into object and primitive fields?
    - other?

## Missing features
- Proper charset support
- `invokedynamic` (Java 8 lambdas) support
- Multithreading (`Thread.start`) support
- Lots of API
- Proper garbage collection
- `no_std` support
- A JIT compiler
