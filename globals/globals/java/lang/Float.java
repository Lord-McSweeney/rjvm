package java.lang;

public final class Float extends Number {
    public static Class<Float> TYPE = (Class<Float>) Class.getPrimitiveClass(Class.PRIM_FLOAT);

    private float value;

    public Float(float value) {
        this.value = value;
    }

    // TODO: Implement equals and compareTo with NaN comparison rules

    public float floatValue() {
        return this.value;
    }

    public static Float valueOf(float f) {
        return new Float(f);
    }
}
