package java.lang;

public final class Float extends Number {
    public static Class<Float> TYPE = (Class<Float>) Class.getPrimitiveClass(Class.PRIM_FLOAT);

    private float value;

    public Float(float value) {
        this.value = value;
    }

    // TODO: Implement equals and compareTo with NaN comparison rules

    public int intValue() {
        return (int) this.value;
    }

    public float floatValue() {
        return this.value;
    }

    public long longValue() {
        return (long) this.value;
    }

    public double doubleValue() {
        return (double) this.value;
    }

    public static Float valueOf(float f) {
        return new Float(f);
    }

    public String toString() {
        return Float.toString(this.value);
    }

    // FIXME: Implement this correctly
    public static native String toString(float f);
}
