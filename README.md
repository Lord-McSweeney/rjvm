# Java Virtual Machine implementation in Rust

## Features
- bytecode verifier
- support for some classes in the `java.io`, `java.lang`, and `java.util` packages
- ability to run Java on the browser through WASM

## Upcoming features
- Implementations for the remaining stack operations
- Proper `ClassLoader`s
    - Finding classes in the current directory when running a class
- Array type verification in verifier
- Object class verification in verifier
- Better performance
    - 8-byte Value
    - don't allocate on call
    - split fields into object and primitive fields?
    - other?

## Missing features
- Multithreading (`Thread.start`) support
- Lots of API
- Proper garbage collection
- `no_std` support
- A JIT compiler
