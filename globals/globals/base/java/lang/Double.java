package java.lang;

import rjvm.internal.Todo;

public final class Double extends Number {
    public static Class<Double> TYPE = (Class<Double>) Class.getPrimitiveClass(Class.PRIM_DOUBLE);

    public static final double NEGATIVE_INFINITY = -1.0 / 0.0;
    public static final double POSITIVE_INFINITY = 1.0 / 0.0;

    private double value;

    public Double(double value) {
        this.value = value;
    }

    // TODO: Implement equals and compareTo with NaN comparison rules

    public double doubleValue() {
        return this.value;
    }

    // Utility functions
    public boolean isInfinite() {
        return Double.isInfinite(this.value);
    }

    public static boolean isInfinite(double value) {
        return value == NEGATIVE_INFINITY || value == POSITIVE_INFINITY;
    }

    public boolean isNaN() {
        return Double.isNaN(value);
    }

    public static boolean isNaN(double value) {
        return value != value;
    }

    public static Double valueOf(double d) {
        return new Double(d);
    }

    public static Double valueOf(String string) {
        return Double.parseDouble(string);
    }

    // Access to raw bits of doubles
    public static long doubleToLongBits(double value) {
        if (Double.isNaN(value)) {
            return  0x7ff8000000000000L;
        } else {
            return Double.doubleToRawLongBits(value);
        }
    }

    public static native long doubleToRawLongBits(double value);

    // String operation functions
    public static double parseDouble(String string) throws NumberFormatException {
        Todo.warnNotImpl("java.lang.Double.parseDouble");

        return 0.0;
    }

    public static String toString(double d) {
        Todo.warnNotImpl("java.lang.Double.toString");

        return "0.0";
    }
}
