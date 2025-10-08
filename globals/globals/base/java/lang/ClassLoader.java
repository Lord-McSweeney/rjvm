package java.lang;

public abstract class ClassLoader {
    private ClassLoader parent;

    protected ClassLoader() {
        super();
    }

    protected ClassLoader(ClassLoader parent) {
        this.parent = parent;
    }
}
