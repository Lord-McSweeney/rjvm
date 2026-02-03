package java.lang.reflect;

public final class Method extends Executable {
    private Method() { }

    public native Class<?> getDeclaringClass();

    public native String getName();

    public native int getParameterCount();

    public Object invoke(Object obj, Object... args) throws IllegalAccessException, IllegalArgumentException, InvocationTargetException {
        if (args == null) {
            args = new Object[0];
        }

        return this.invokeNative(obj, args);
    }

    private native Object invokeNative(Object obj, Object[] args) throws IllegalAccessException, IllegalArgumentException, InvocationTargetException;
}
