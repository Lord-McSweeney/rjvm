# globals

`rjvm_globals` stores builtin classes and has some native method implementations.

If you want to use `rjvm_globals` yourself, you will need to provide implementations for some native methods:
- `java/lang/Runtime.exit.(I)V`
- `java/lang/System.currentTimeMillis.()J`
- `java/io/File.internalInitFileData.([B)V`
- `java/io/File.getCanonicalPath.()Ljava/lang/String;`
- `java/io/File.getAbsolutePath.()Ljava/lang/String;`
- `java/io/FileOutputStream.writeInternal.(I)V`
- `java/io/FileOutputStream.flushInternal.()V`
- `java/io/FileInputStream.readInternal.()I`
- `java/io/FileInputStream.readMultiInternal.([BII)I`
- `java/io/FileInputStream.availableInternal.()I`
- `java/io/FileDescriptor.internalWriteableDescriptorFromPath.(Ljava/lang/String;)I`
- `java/io/FileDescriptor.internalReadableDescriptorFromPath.(Ljava/lang/String;)I`

Otherwise the JVM will panic when those methods are called. Take a look at `web/src/native_impl.rs` for an example of how to do so.

IMPORTANT:
If you want to replace or edit the classes defined here, be sure to keep the following classes:
- `java/lang/ArithmeticException`
- `java/lang/ArrayIndexOutOfBoundsException`
- `java/lang/ArrayStoreException`
- `java/lang/Class`
- `java/lang/ClassCastException`
- `java/lang/CloneNotSupportedException`
- `java/lang/Cloneable`
- `java/lang/IllegalAccessError`
- `java/lang/IncompatibleClassChangeError`
- `java/lang/InstantiationError`
- `java/lang/InstantiationException`
- `java/lang/NegativeArraySizeException`
- `java/lang/NoClassDefFoundError`
- `java/lang/NoSuchFieldError`
- `java/lang/NoSuchMethodError`
- `java/lang/NullPointerException`
- `java/lang/Object`
- `java/lang/StackTraceElement`
- `java/lang/String`
- `java/lang/System`
- `java/lang/Throwable`
- `java/lang/VerifyError`
- `java/lang/reflect/Constructor`
- `java/lang/reflect/Method`

These are critical to the JVM and it will panic on startup if they are missing.

Additionally:
- The first static method of the `System` class will be called immediately after VM startup. This method is responsible for calling `ClassLoader.getSystemClassLoader()`. In turn, the first time `getSystemClassLoader` is called, it is responsible for setting the system class loader on the native `Context` using `Context::init_system_loader`. If the system class loader is not set, attempting to access it will cause the VM to panic.
- The first static method of the `StackTraceElement` class will be called to create a `StackTraceElement` from a `java.lang.reflect.Executable`. It should take one `Executable` as a parameter and return a `StackTraceElement`.
- Various other methods need to be ordered correctly (e.g. `Object.toString`, `Throwable.message`, `Throwable.stackTrace`).
- Several classes that represent VM objects need to have an `int` field as their first instance field. This field will be set by the VM when they are created and is used as a unique ID for them.
