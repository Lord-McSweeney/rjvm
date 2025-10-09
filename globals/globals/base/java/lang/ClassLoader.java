package java.lang;

import java.io.InputStream;

public abstract class ClassLoader {
    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;
    private ClassLoader parent;

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

    public static native ClassLoader getSystemClassLoader();

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
