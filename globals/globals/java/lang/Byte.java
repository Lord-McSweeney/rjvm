package java.lang;

public final class Byte extends Number implements Comparable<Byte> {
    public static Class<Byte> TYPE = (Class<Byte>) Class.getPrimitiveClass(Class.PRIM_BYTE);

    private byte value;

    public Byte(byte value) {
        this.value = value;
    }

    public boolean equals(Object obj) {
        if (obj instanceof Byte) {
            Byte other = (Byte) obj;
            return this.value == other.value;
        } else {
            return false;
        }
    }

    public byte byteValue() {
        return this.value;
    }

    public static Byte valueOf(byte b) {
        return new Byte(b);
    }

    public int compareTo(Byte other) {
        if (this.value < other.value) {
            return -1;
        } else if (this.value == other.value) {
            return 0;
        } else {
            return 1;
        }
    }
}
