rjvm is a performant Java Virtual Machine implementation in Rust.

Features:
- bytecode verifier
- support for many common operations
- support for some classes in the `java.io`, `java.lang`, and `java.util` packages

Missing features:
- multithreading/`synchronized` support
- support for several operations, especially around doubles and floats
- lots of API
- proper garbage collection

Upcoming features:
- Array type verification in verifier
- Object class verification in verifier
- Storage specialization for primitive-type arrays
