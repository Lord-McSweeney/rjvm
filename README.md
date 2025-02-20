rjvm is a Java Virtual Machine implementation in Rust.

Features:
- bytecode verifier
- support for most common operations
- support for several classes in the java.io, java.lang, java.nio, and java.util packages

Missing features:
- multithreading/`synchronized` support
- support for several operations, especially around doubles and floats

Upcoming features:
- Array type verification in verifier
- Storage specialization for primitive-type arrays
- Early binding for `invokevirtual`
