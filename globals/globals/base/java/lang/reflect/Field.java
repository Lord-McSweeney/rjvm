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
                throw new Error("TODO - static float field get");
            } else if (typeCls == int.class) {
                throw new Error("TODO - static int field get");
            } else if (typeCls == long.class) {
                throw new Error("TODO - static long field get");
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
                throw new Error("TODO - instance float field get");
            } else if (typeCls == int.class) {
                throw new Error("TODO - instance int field get");
            } else if (typeCls == long.class) {
                throw new Error("TODO - instance long field get");
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
    static native Object getObjectStaticNative(Field field);
    static native Object getObjectInstanceNative(Field field, Object receiver);
}
