package java.lang.reflect;

public final class Method extends Executable {
    private Method() { }

    public native Class<?> getDeclaringClass();

    public native String getName();

    public native int getParameterCount();
}
