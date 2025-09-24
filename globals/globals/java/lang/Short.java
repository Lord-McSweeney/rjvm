package java.lang;

public final class Short extends Number implements Comparable<Short> {
    public static Class<Short> TYPE = (Class<Short>) Class.getPrimitiveClass(Class.PRIM_SHORT);

    private short value;

    public Short(short value) {
        this.value = value;
    }

    public boolean equals(Object obj) {
        if (obj instanceof Short) {
            Short other = (Short) obj;
            return this.value == other.value;
        } else {
            return false;
        }
    }

    public short shortValue() {
        return this.value;
    }

    public static Short valueOf(short s) {
        return new Short(s);
    }

    public int compareTo(Short other) {
        if (this.value < other.value) {
            return -1;
        } else if (this.value == other.value) {
            return 0;
        } else {
            return 1;
        }
    }
}
