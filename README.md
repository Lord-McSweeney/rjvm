# Java Virtual Machine implementation in Rust

## Features
- bytecode verifier
- support for some classes in the `java.io`, `java.lang`, and `java.util` packages
- ability to run Java on the browser through WASM

## Upcoming features
- Implementations for the remaining conversion and stack operations
- Proper `ClassLoader`s
    - Finding classes in the current directory when running a class
- Array type verification in verifier
- Object class verification in verifier

## Missing features
- multithreading (`Thread.start`) support
- lots of API
- proper garbage collection
- `no_std` support
