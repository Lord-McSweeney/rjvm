package java.lang.reflect;

public final class Array {
    public static Object newInstance​(Class<?> componentType, int length) throws NegativeArraySizeException {
        if (componentType == null) {
            throw new NullPointerException();
        } else if (length < 0) {
            throw new NegativeArraySizeException();
        } else if (componentType == void.class) {
            // Can't have a void array
            throw new IllegalArgumentException();
        }

        return Array.newInstanceNative(componentType, length);
    }

    private static native Object newInstanceNative​(Class<?> componentType, int length);
}
