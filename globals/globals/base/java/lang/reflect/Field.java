package java.lang.reflect;

public final class Field extends AccessibleObject implements Member {
    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;

    private Field() { }

    public native Class<?> getDeclaringClass();

    public native String getName();

    public native Class<?> getType();
}
