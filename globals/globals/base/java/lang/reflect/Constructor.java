package java.lang.reflect;

public final class Constructor<T> extends Executable {
    public T newInstance(Object... args) {
        if (args == null) {
            args = new Object[0];
        }

        return this.newInstanceNative(args);
    }

    private native T newInstanceNative(Object[] args);

    public native int getParameterCount();
}
