package java.lang;

public class Boolean implements Comparable<Boolean> {
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

    public boolean booleanValue() {
        return value;
    }

    public String toString() {
        if (this.value) {
            return "true";
        } else {
            return "false";
        }
    }
}
