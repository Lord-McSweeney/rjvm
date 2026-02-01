package java.lang.reflect;

public final class Constructor<T> extends Executable {
    // `Constructor` is only ever natively constructed (`Object::constructor_object`)
    private Constructor() { }

    public T newInstance(Object... args) throws InstantiationException, IllegalAccessException, IllegalArgumentException, InvocationTargetException {
        if (args == null) {
            args = new Object[0];
        }

        return this.newInstanceNative(args);
    }

    private native T newInstanceNative(Object[] args) throws InstantiationException, IllegalAccessException, IllegalArgumentException, InvocationTargetException;

    public native Class<?> getDeclaringClass();

    public String getName() {
        return this.getDeclaringClass().getName();
    }

    public native int getParameterCount();
}
