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

    public Double(String string) throws NumberFormatException {
        this(Double.parseDouble(string));
    }

    public static Double valueOf(double d) {
        return new Double(d);
    }

    public static Double valueOf(String string) {
        return Double.parseDouble(string);
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

    // Access to raw bits of doubles
    public static long doubleToLongBits(double value) {
        if (Double.isNaN(value)) {
            return 0x7ff8000000000000L;
        } else {
            return Double.doubleToRawLongBits(value);
        }
    }

    public static native long doubleToRawLongBits(double value);

    // String operation functions
    public static double parseDouble(String string) throws NumberFormatException {
        string = string.trim();

        if (string.length() == 0) {
            throw new NumberFormatException();
        }

        if (string.equals("NaN") || string.equals("-NaN")) {
            return 0.0 / 0.0;
        } else if (string.equals("Infinity")) {
            return 1.0 / 0.0;
        } else if (string.equals("-Infinity")) {
            return -1.0 / 0.0;
        }

        double result = 0.0;
        int position = 0;
        boolean isNeg = false;

        char firstChar = string.charAt(0);
        if (firstChar == '-') {
            isNeg = true;
            position += 1;

            if (string.length() == 1) {
                // Cannot just have "+"
                throw new NumberFormatException();
            }
        } else if (string.charAt(0) == '+') {
            // No need to do anything
            position += 1;

            if (string.length() == 1) {
                // Cannot just have "-"
                throw new NumberFormatException();
            }
        }

        int offset = 0;

        // TODO: Are there any more rules left to implement?

        int firstDotIndex = string.indexOf(".");
        if (firstDotIndex == -1) {
            firstDotIndex = string.length();
        }
        double multiplier;
        while (position < string.length()) {
            multiplier = Math.pow(10, firstDotIndex - (position - offset) - 1);
            if (position == firstDotIndex) {
                // Skip dot
                offset += 1;
                position += 1;
            }

            char curChar = string.charAt(position);

            if (curChar == 'e') {
                char[] pow10Chars = new char[string.length() - position - 1];
                string.getChars(position + 1, string.length(), pow10Chars, 0);

                String pow10String = new String(pow10Chars);
                int pow10Amount = Integer.parseInt(pow10String);

                return result * Math.pow(10.0, pow10Amount);
            }

            if (curChar == 'f' || curChar == 'd') {
                if (position == string.length() - 1) {
                    // If this string ends with `f` or `d`, ignore it
                    break;
                }
            }

            if (curChar < '0' || curChar > '9') {
                throw new NumberFormatException();
            }

            result += (curChar - '0') * multiplier;

            position += 1;
        }

        if (isNeg) {
            result = -result;
        }

        return result;
    }

    // FIXME: Implement this correctly
    public static native String toString(double d);
}
