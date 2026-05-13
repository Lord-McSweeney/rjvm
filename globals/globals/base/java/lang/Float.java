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

    public static Float valueOf(float f) {
        return new Float(f);
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

    // Utility functions
    public boolean isInfinite() {
        return Float.isInfinite(this.value);
    }

    public static boolean isInfinite(float value) {
        return value == NEGATIVE_INFINITY || value == POSITIVE_INFINITY;
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

    // String operation functions
    public static float parseFloat(String string) throws NumberFormatException {
        string = string.trim();

        if (string.length() == 0) {
            throw new NumberFormatException();
        }

        if (string.equals("NaN") || string.equals("-NaN")) {
            return 0.0f / 0.0f;
        } else if (string.equals("Infinity")) {
            return 1.0f / 0.0f;
        } else if (string.equals("-Infinity")) {
            return -1.0f / 0.0f;
        }

        float result = 0.0f;
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
        float multiplier;
        while (position < string.length()) {
            multiplier = (float) Math.pow(10, firstDotIndex - (position - offset) - 1);
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

                return result * (float) Math.pow(10.0, pow10Amount);
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

    public String toString() {
        return Float.toString(this.value);
    }

    // FIXME: Implement this correctly
    public static native String toString(float f);
}
