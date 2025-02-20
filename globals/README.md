# globals

`rjvm_globals` stores builtin classes and has some native method implementations.

IMPORTANT:
If you want to replace or edit the classes defined here, be sure to keep the following classes:
- `java/lang/ArithmeticException`
- `java/lang/ArrayIndexOutOfBoundsException`
- `java/lang/Class`
- `java/lang/ClassCastException`
- `java/lang/NegativeArraySizeException`
- `java/lang/NoClassDefFoundError`
- `java/lang/NullPointerException`
- `java/lang/Object`
- `java/lang/String`
- `java/lang/Throwable`

These are critical to the JVM and it may panic if they are missing.
