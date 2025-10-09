package java.lang;

public abstract class ClassLoader {
    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;
    private ClassLoader parent;

    protected ClassLoader() {
        this(ClassLoader.getSystemClassLoader());
    }

    protected ClassLoader(ClassLoader parent) {
        this.internalId = -1;
        this.parent = parent;
    }

    public ClassLoader getParent() {
        return this.parent;
    }

    public static native ClassLoader getSystemClassLoader();
}
