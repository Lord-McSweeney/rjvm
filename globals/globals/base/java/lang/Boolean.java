package java.lang;

public class Boolean implements Comparable<Boolean> {
    public static Class<Boolean> TYPE = (Class<Boolean>) Class.getPrimitiveClass(Class.PRIM_BOOLEAN);

    public static Boolean TRUE = new Boolean(true);
    public static Boolean FALSE = new Boolean(false);

    private boolean value;

    public Boolean(boolean value) {
        this.value = value;
    }

    public static Boolean valueOf(boolean value) {
        if (value) {
            return Boolean.TRUE;
        } else {
            return Boolean.FALSE;
        }
    }

    public static boolean parseBoolean(String string) {
        if (string != null) {
            // TODO this should be `equalsIgnoreCase`, not `equals`
            return string.equals("true");
        } else {
            return false;
        }
    }

    public boolean booleanValue() {
        return this.value;
    }

    public String toString() {
        if (this.value) {
            return "true";
        } else {
            return "false";
        }
    }

    public int compareTo(Boolean other) {
        if (this.value == other.value) {
            return 0;
        } else if (this.value) {
            // (true).compareTo(false)
            return 1;
        } else {
            // (false).compareTo(true)
            return -1;
        }
    }
}
