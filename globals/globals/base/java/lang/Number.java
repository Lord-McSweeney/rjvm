package java.lang;

public abstract class Number {
    public byte byteValue() {
        return (byte) this.intValue();
    }

    public short shortValue() {
        return (short) this.intValue();
    }

    public abstract int intValue();
}
