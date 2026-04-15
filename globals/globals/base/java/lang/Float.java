package java.lang;

public final class Float extends Number {
    public static Class<Float> TYPE = (Class<Float>) Class.getPrimitiveClass(Class.PRIM_FLOAT);

    public static final float POSITIVE_INFINITY = 1.0f / 0.0f;
    public static final float NEGATIVE_INFINITY = -1.0f / 0.0f;
    public static final float NaN = 0.0f / 0.0f;

    public static final float MAX_VALUE = 0x1.fffffeP+127f;
    public static final float MIN_NORMAL = 0x1.0p-126f;
    public static final float MIN_VALUE = 0x0.000002P-126f;

    public static final int MAX_EXPONENT = 127;
    public static final int MIN_EXPONENT = -126;
    public static final int SIZE = 32;

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

    public boolean isNaN() {
        return Float.isNaN(this.value);
    }

    public static boolean isNaN(float value) {
        return value != value;
    }

    // Access to raw bits of floats
    public static int floatToIntBits(float value) {
        if (Float.isNaN(value)) {
            return 0x7fc00000;
        } else {
            return Float.floatToRawIntBits(value);
        }
    }

    public static native int floatToRawIntBits(float value);

    public String toString() {
        return Float.toString(this.value);
    }

    // FIXME: Implement this correctly
    public static native String toString(float f);
}
