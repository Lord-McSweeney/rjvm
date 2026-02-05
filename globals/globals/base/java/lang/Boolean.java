package java.lang;

public final class Boolean implements Comparable<Boolean> {
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

    // Return the boolean system property with the given name
    public static boolean getBoolean(String name) {
        try {
            return Boolean.parseBoolean(System.getProperty(name));
        } catch (IllegalArgumentException e) {

        } catch (NullPointerException e) {

        }
        return false;
    }

    public static boolean parseBoolean(String string) {
        if (string != null) {
            // TODO this should be `equalsIgnoreCase`, not `equals`
            return string.equals("true");
        } else {
            return false;
        }
    }

    public static String toString(boolean b) {
        if (b) {
            return "true";
        } else {
            return "false";
        }
    }

    public boolean booleanValue() {
        return this.value;
    }

    public String toString() {
        return Boolean.toString(this.value);
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
