package java.lang.reflect;

public final class Field extends AccessibleObject implements Member {
    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;

    private Field() { }

    public native Class<?> getDeclaringClass();

    public native String getName();

    public native Class<?> getType();

    public native int getModifiers();

    public Object get(Object receiver) throws IllegalArgumentException, IllegalAccessException {
        boolean isStatic = Modifier.isStatic(this.getModifiers());

        Class<?> fieldCls = this.getDeclaringClass();
        Class<?> typeCls = this.getType();

        if (isStatic) {
            if (typeCls == boolean.class) {
                throw new Error("TODO - static boolean field get");
            } else if (typeCls == byte.class) {
                throw new Error("TODO - static byte field get");
            } else if (typeCls == char.class) {
                throw new Error("TODO - static char field get");
            } else if (typeCls == double.class) {
                throw new Error("TODO - static double field get");
            } else if (typeCls == float.class) {
                return FieldAccess.getFloatStaticNative(this);
            } else if (typeCls == int.class) {
                return FieldAccess.getIntStaticNative(this);
            } else if (typeCls == long.class) {
                return FieldAccess.getLongStaticNative(this);
            } else if (typeCls == short.class) {
                throw new Error("TODO - static short field get");
            } else {
                return FieldAccess.getObjectStaticNative(this);
            }
        } else {
            if (receiver == null) {
                throw new NullPointerException();
            } else if (!fieldCls.isInstance(receiver)) {
                // Not the right type of receiver
                throw new IllegalArgumentException();
            }

            if (typeCls == boolean.class) {
                throw new Error("TODO - instance boolean field get");
            } else if (typeCls == byte.class) {
                throw new Error("TODO - instance byte field get");
            } else if (typeCls == char.class) {
                throw new Error("TODO - instance char field get");
            } else if (typeCls == double.class) {
                throw new Error("TODO - instance double field get");
            } else if (typeCls == float.class) {
                return FieldAccess.getFloatInstanceNative(this, receiver);
            } else if (typeCls == int.class) {
                return FieldAccess.getIntInstanceNative(this, receiver);
            } else if (typeCls == long.class) {
                return FieldAccess.getLongInstanceNative(this, receiver);
            } else if (typeCls == short.class) {
                throw new Error("TODO - instance short field get");
            } else {
                return FieldAccess.getObjectInstanceNative(this, receiver);
            }
        }
    }
}

// A class containing simple native implementations for field access.
class FieldAccess {
    static native float getFloatStaticNative(Field field);
    static native float getFloatInstanceNative(Field field, Object receiver);

    static native int getIntStaticNative(Field field);
    static native int getIntInstanceNative(Field field, Object receiver);

    static native long getLongStaticNative(Field field);
    static native long getLongInstanceNative(Field field, Object receiver);

    static native Object getObjectStaticNative(Field field);
    static native Object getObjectInstanceNative(Field field, Object receiver);
}
