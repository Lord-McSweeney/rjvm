package java.lang.reflect;

public final class Method extends Executable {
    private Method() { }

    public native Class<?> getDeclaringClass();

    public native String getName();

    public native int getParameterCount();

    public Object invoke(Object obj, Object... args) throws IllegalAccessException, IllegalArgumentException, InvocationTargetException {
        // TODO implement
        return null;
    }
}
