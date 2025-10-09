package java.lang;

import java.lang.reflect.AnnotatedElement;
import java.lang.reflect.Constructor;
import java.lang.reflect.GenericDeclaration;
import java.lang.reflect.Type;

import java.io.InputStream;

// NOTE: The native `Class` corresponding to this `Class<T>` is stored in the
// native `Context` and can be retrieved with `Context::class_for_java_class`
public final class Class<T> implements AnnotatedElement, GenericDeclaration, Type {
    static final int PRIM_BOOLEAN = 0;
    static final int PRIM_BYTE = 1;
    static final int PRIM_CHAR = 2;
    static final int PRIM_SHORT = 3;
    static final int PRIM_INT = 4;
    static final int PRIM_LONG = 5;
    static final int PRIM_FLOAT = 6;
    static final int PRIM_DOUBLE = 7;
    static final int PRIM_VOID = 8;

    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;
    private String cachedName;

    private Class() { }

    public String getName() {
        if (this.cachedName == null) {
            String name = this.getNameNative();
            this.cachedName = name;
        }

        return this.cachedName;
    }
    private native String getNameNative();

    public native boolean isInterface();

    public native boolean isPrimitive();

    public native ClassLoader getClassLoader();

    public InputStream getResourceAsStream(String resourceName) {
        // Resolve the name relative to this class's name. For example, if in
        // `com/example/MyClass`, looking up `rsrc.txt` looks up
        // `com/example/rsrc.txt`.
        if (resourceName != null) {
            resourceName = this.getAbsoluteName(resourceName);
        }

        ClassLoader loader = this.getClassLoader();

        if (loader == null) {
            return ClassLoader.getSystemResourceAsStream(resourceName);
        } else {
            return loader.getResourceAsStream(resourceName);
        }
    }

    public boolean desiredAssertionStatus() {
        // TODO implement (this isn't very important)
        return false;
    }

    static native Class<?> getPrimitiveClass(int id);

    public static Class<?> forName(String className) throws ClassNotFoundException {
        if (className == null) {
            throw new NullPointerException();
        }

        Class<?> result = Class.forNameNative(className);
        if (result == null) {
            throw new ClassNotFoundException();
        } else {
            return result;
        }
    }
    private static native Class<?> forNameNative(String className);

    private native String getAbsoluteName(String path);

    public native Constructor<?>[] getConstructors();

    public String toString() {
        if (this.isPrimitive()) {
            // Primitive classes just return their name
            return this.getName();
        } else {
            StringBuilder result = new StringBuilder();

            if (this.isInterface()) {
                result.append("interface ");
            } else {
                result.append("class ");
            }

            result.append(this.getName());

            return result.toString();
        }
    }
}
