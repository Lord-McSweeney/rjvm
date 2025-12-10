package java.util;

public final class Objects {
    private Objects() { }

    public static boolean equals(Object a, Object b) {
        if (a == b) {
            return true;
        } else if (a == null) {
            return false;
        } else if (a.equals(b)) {
            return true;
        } else {
            return false;
        }
    }

    public static int hashCode(Object obj) {
        if (obj == null) {
            return 0;
        } else {
            return obj.hashCode();
        }
    }

    public static String toString(Object obj) {
        return String.valueOf(obj);
    }
}
