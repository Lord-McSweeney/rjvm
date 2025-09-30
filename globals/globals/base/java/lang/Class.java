package java.lang;

import java.lang.reflect.AnnotatedElement;
import java.lang.reflect.GenericDeclaration;
import java.lang.reflect.Type;
import java.io.ByteArrayInputStream;
import java.io.IOException;
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

    private Class() { }

    private String cachedName;
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

    public InputStream getResourceAsStream(String resourceName) {
        if (resourceName == null) {
            throw new NullPointerException();
        }

        byte[] resourceData = this.getResourceData(resourceName);

        if (resourceData != null) {
            return new ByteArrayInputStream(resourceData);
        } else {
            return null;
        }
    }
    private native byte[] getResourceData(String resourceName);

    public boolean desiredAssertionStatus() {
        // TODO implement
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
