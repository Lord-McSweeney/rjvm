package java.lang;

import java.lang.reflect.AnnotatedElement;
import java.lang.reflect.Constructor;
import java.lang.reflect.GenericDeclaration;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Type;
import java.lang.reflect.TypeVariable;

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

    public boolean isArray() {
        return this.getComponentType() != null;
    }

    public boolean isEnum() {
        boolean hasEnumModifier = (this.getModifiers() & 0x4000) != 0;
        boolean extendsEnum = this.getSuperclass() == Enum.class;

        // Enum class must have the modifier *and* extend `Enum`
        return hasEnumModifier && extendsEnum;
    }

    public boolean isInterface() {
        return (this.getModifiers() & 0x200) != 0;
    }

    public native boolean isPrimitive();

    public native Class<?> getComponentType();

    public native Class<? super T> getSuperclass();

    public native Class<?>[] getInterfaces();

    public native ClassLoader getClassLoader();

    public native boolean isInstance(Object obj);

    public native int getModifiers();

    public T[] getEnumConstants() {
        if (!this.isEnum()) {
            return null;
        }

        try {
            Method valuesMethod = this.getMethod("values");
            T[] values = (T[]) valuesMethod.invoke(null);

            if (values == null) {
                return null;
            } else {
                return values.clone();
            }
        } catch (InvocationTargetException ex) {
            return null;
        } catch (NoSuchMethodException ex) {
            return null;
        } catch (IllegalAccessException ex) {
            return null;
        }
    }

    public TypeVariable<Class<T>>[] getTypeParameters() {
        // TODO implement
        return null;
    }

    public InputStream getResourceAsStream(String resourceName) {
        // Resolve the name relative to this class's name. For example, if in
        // `com/example/MyClass`, looking up `rsrc.txt` looks up
        // `com/example/rsrc.txt`.
        resourceName = this.getAbsoluteName(resourceName);

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

    public String getSimpleName() {
        if (this.isArray()) {
            return getComponentType().getSimpleName() + "[]";
        }

        String name = this.getName();
        if (name.indexOf(".") == -1) {
            // Top-level class, return as-is
            return name;
        } else {
            // Skip package name
            return name.substring(name.lastIndexOf(".") + 1);
        }
    }

    private String getAbsoluteName(String path) {
        if (path == null) {
            return path;
        }

        if (path.startsWith("/")) {
            // Absolute path
            return path.substring(1);
        } else {
            // Get innermost type
            Class<?> clazz = this;
            while (clazz.isArray()) {
                clazz = clazz.getComponentType();
            }
            String baseName = clazz.getName();

            // Resolve name
            int index = baseName.lastIndexOf('.');
            if (index != -1) {
                String basePath = baseName.substring(0, index).replace('.', '/');
                return basePath + '/' + path;
            } else {
                return path;
            }
        }
    }

    // Reflection methods

    public static Class<?> forName(String className) throws ClassNotFoundException {
        // FIXME implement `Reflection.getCallerClass` so we can use the loader
        // of the class of the caller method
        return ClassLoader.getSystemClassLoader().loadClass(className);
    }

    public native Constructor<?>[] getConstructors();

    public native Method[] getDeclaredMethods();

    public Method getMethod(String name, Class<?>... parameterTypes) throws NoSuchMethodException {
        if (name == null) {
            throw new NullPointerException();
        }

        // Special-case: init and clinit aren't found
        if (name.equals("<init>") || name.equals("<clinit>")) {
            throw new NoSuchMethodException();
        }

        // Special-case: clone method isn't found on arrays
        if (name == "clone" && this.isArray()) {
            throw new NoSuchMethodException();
        }

        // If `null` is passed, try to find a method with no parameters
        if (parameterTypes == null) {
            parameterTypes = new Class<?>[0];
        }

        Method result = this.getMethodNative(name, parameterTypes);
        if (result == null) {
            throw new NoSuchMethodException();
        } else {
            return result;
        }
    }
    private native Method getMethodNative(String name, Class[] parameterTypes);

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
