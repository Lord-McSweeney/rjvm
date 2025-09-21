# globals

`rjvm_globals` stores builtin classes and has some native method implementations.

If you want to use `rjvm_globals` yourself, you will need to provide implementations for some native methods:
- `java/lang/Runtime.exit.(I)V`
- `java/lang/System.nanoTime.()J`
- `java/io/File.internalInitFileData.([B)V`
- `java/io/File.getCanonicalPath.()Ljava/lang/String;`
- `java/io/File.getAbsolutePath.()Ljava/lang/String;`
- `java/io/FileOutputStream.writeInternal.(I)V`
- `java/io/FileInputStream.readInternal.()I`
- `java/io/FileInputStream.availableInternal.()I`
- `java/io/FileDescriptor.internalWriteableDescriptorFromPath.(Ljava/lang/String;)I`
- `java/io/FileDescriptor.internalReadableDescriptorFromPath.(Ljava/lang/String;)I`

Otherwise the JVM will panic. Take a look at `web/src/native_impl.rs` for an example of how to do so.

IMPORTANT:
If you want to replace or edit the classes defined here, be sure to keep the following classes:
- `java/lang/ArithmeticException`
- `java/lang/ArrayIndexOutOfBoundsException`
- `java/lang/ArrayStoreException`
- `java/lang/Class`
- `java/lang/ClassCastException`
- `java/lang/CloneNotSupportedException`
- `java/lang/Cloneable`
- `java/lang/NegativeArraySizeException`
- `java/lang/NoClassDefFoundError`
- `java/lang/NoSuchFieldError`
- `java/lang/NoSuchMethodError`
- `java/lang/NullPointerException`
- `java/lang/Object`
- `java/lang/String`
- `java/lang/Throwable`

These are critical to the JVM and it may panic if they are missing.
