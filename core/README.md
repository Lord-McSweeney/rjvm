# core

`rjvm_core` is where all the JVM code resides. The code is divided into five subdirectories:
- `classfile`, for parsing class files
- `gc`, which implements a primitive tracing garbage collector
- `jar`, for loading JAR files
- `runtime`, for running class files
- `string`, which implements very simple garbage-collected UTF-8 strings
