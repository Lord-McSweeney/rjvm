package java.lang;

import java.io.InputStream;

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
        this.internalId = -1;
        this.parent = parent;
    }

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
