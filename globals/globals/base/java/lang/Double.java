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

    public boolean isInfinite() {
        return this.value == NEGATIVE_INFINITY || this.value == POSITIVE_INFINITY;
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

    public static double parseDouble(String string) throws NumberFormatException {
        Todo.warnNotImpl("java.lang.Double.parseDouble");

        return 0.0;
    }
}
