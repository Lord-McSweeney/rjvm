package java.lang;

import java.io.InputStream;
import java.security.ProtectionDomain;

// For the system class loader
import jvm.internal.ClassLoaderUtils;

public abstract class ClassLoader {
    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;
    private ClassLoader parent;

    private static ClassLoader systemClassLoader;

    // Constructors
    protected ClassLoader() {
        this(ClassLoader.getSystemClassLoader());
    }

    protected ClassLoader(ClassLoader parent) {
        this.parent = parent;

        this.registerAsLoader(parent);
    }

    private native void registerAsLoader(ClassLoader parent);

    // Misc functions
    public ClassLoader getParent() {
        return this.parent;
    }

    public static ClassLoader getSystemClassLoader() {
        // This method is responsible for creating the system class loader the
        // first time it's called
        if (ClassLoader.systemClassLoader == null) {
            ClassLoader platformLoader = ClassLoaderUtils.createPlatformLoader();
            ClassLoader systemLoader = ClassLoaderUtils.createSystemLoader(platformLoader);

            ClassLoader.systemClassLoader = systemLoader;
        }

        return ClassLoader.systemClassLoader;
    }

    protected Class<?> loadClass(String name, boolean resolve) throws ClassNotFoundException {
        // The `resolve` parameter is ignored- `ClassLoader.loadClass` always links the class,
        // even if the parameter is set to `false`
        return this.loadClass(name);
    }

    // Functions to load classes
    public Class<?> loadClass(String className) throws ClassNotFoundException {
        // TODO implement `findLoadedClass`
        Class<?> cls = findLoadedClass(className);
        if (cls == null) {
            try {
                if (parent == null) {
                    if (ClassLoader.isValidClassName(className)) {
                        if (className != null) {
                            // Bootstrap loader
                            cls = ClassLoader.loadBootstrapClassNative(className);
                        }
                    }
                } else {
                    // Resolve parameter is ignored
                    cls = this.parent.loadClass(className, false);
                }
                // TODO try to load a bootstrap class...?
            } catch (ClassNotFoundException e) {
                // CNFE silently ignored
            }

            if (cls == null) {
                cls = this.findClass(className);
            }
        }

        // `resolveClass` is a noop so no need to call it

        return cls;
    }

    private static native Class<?> loadBootstrapClassNative(String name);

    protected Class<?> findClass(String name) throws ClassNotFoundException {
        throw new ClassNotFoundException(name);
    }

    protected final Class<?> findLoadedClass(String name) {
        if (!ClassLoader.isValidClassName(name)) {
            return null;
        }

        return this.findLoadedClassNative(name);
    }
    private native Class<?> findLoadedClassNative(String name);

    protected final void resolveClass(Class<?> cls) {
        // Despite what the documentation says, this method is actually a no-op
        // (besides the null check)
        if (cls == null) {
            throw new NullPointerException();
        }
    }

    private static boolean isValidClassName(String name) {
        if (name == null || name.length() == 0) {
            return true;
        }

        if (name.indexOf('/') != -1) {
            return false;
        }

        // Array classes are never found by class loaders
        if (name.charAt(0) == '[') {
            return false;
        }

        return true;
    }

    // Functions to define classes
    protected final Class<?> defineClass(byte[] b, int off, int len) throws ClassFormatError {
        if (b == null) {
            throw new NullPointerException();
        }
        if (off < 0 || len < 0 || off + len > b.length) {
            throw new IndexOutOfBoundsException();
        }

        return this.defineClassNative(b, off, len);
    }
    protected final native Class<?> defineClassNative(byte[] b, int off, int len) throws ClassFormatError;

    // This is like `defineClass(byte[], int, int)`, but throws NCDFE if the
    // name of the class doesn't match the given name
    protected final Class<?> defineClass(String name, byte[] b, int off, int len) throws ClassFormatError {
        if (name == null || b == null) {
            throw new NullPointerException();
        }
        if (off < 0 || len < 0 || off + len > b.length) {
            throw new IndexOutOfBoundsException();
        }

        return this.defineClassNative(name, b, off, len);
    }
    protected final native Class<?> defineClassNative(String name, byte[] b, int off, int len) throws ClassFormatError;

    protected final Class<?> defineClass(String name, byte[] b, int off, int len, ProtectionDomain protectionDomain) throws ClassFormatError {
        // Protection domain not yet implemented
        return this.defineClass(name, b, off, len);
    }

    // Functions to get resources
    public static InputStream getSystemResourceAsStream(String resourceName) {
        return ClassLoader.getSystemClassLoader().getResourceAsStream(resourceName);
    }

    public InputStream getResourceAsStream(String resourceName) {
        if (resourceName == null) {
            throw new NullPointerException();
        }

        if (this.parent != null) {
            return this.parent.getResourceAsStream(resourceName);
        } else {
            // TODO search bootstrap loader next
            return null;
        }
    }
}
