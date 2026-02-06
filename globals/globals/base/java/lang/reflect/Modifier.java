package java.lang.reflect;

public class Modifier {
    public static final int PUBLIC       = 0x0001;
    public static final int PRIVATE      = 0x0002;
    public static final int PROTECTED    = 0x0004;
    public static final int STATIC       = 0x0008;
    public static final int FINAL        = 0x0010;
    public static final int SYNCHRONIZED = 0x0020;

    public static boolean isPublic(int mod) {
        return (mod & Modifier.PUBLIC) != 0;
    }

    public static boolean isPrivate(int mod) {
        return (mod & Modifier.PRIVATE) != 0;
    }

    public static boolean isProtected(int mod) {
        return (mod & Modifier.PROTECTED) != 0;
    }

    public static boolean isStatic(int mod) {
        return (mod & Modifier.STATIC) != 0;
    }

    public static boolean isFinal(int mod) {
        return (mod & Modifier.FINAL) != 0;
    }

    public static boolean isSynchronized(int mod) {
        return (mod & Modifier.SYNCHRONIZED) != 0;
    }
}
