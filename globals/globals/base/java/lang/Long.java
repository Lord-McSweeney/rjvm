package java.lang;

public final class Long extends Number implements Comparable<Long> {
    public static Class<Long> TYPE = (Class<Long>) Class.getPrimitiveClass(Class.PRIM_LONG);

    public static final long MIN_VALUE = -9223372036854775808L;
    public static final long MAX_VALUE = 9223372036854775807L;

    private static final char[] ALL_DIGITS = new char[]{
        '0', '1', '2', '3', '4', '5', '6', '7', '8',
        '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    };

    private static final char[] ALL_DIGITS_UPPER = new char[]{
        '0', '1', '2', '3', '4', '5', '6', '7', '8',
        '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    };

    private long value;

    public Long(long value) {
        this.value = value;
    }

    public boolean equals(Object obj) {
        if (obj instanceof Long) {
            Long other = (Long) obj;
            return this.value == other.value;
        } else {
            return false;
        }
    }

    public int hashCode() {
        // TODO
        return ((int) this.value);
    }

    public String toString() {
        return Long.toString(this.value);
    }

    public int intValue() {
        return (int) this.value;
    }

    public float floatValue() {
        return (float) this.value;
    }

    public long longValue() {
        return this.value;
    }

    public double doubleValue() {
        return (double) this.value;
    }

    public static Long valueOf(long integer) {
        return new Long(integer);
    }

    public static String toString(long integer) {
        if (integer == -9223372036854775808L) {
            return "-9223372036854775808";
        }

        if (integer == 0) {
            return "0";
        }

        boolean isNegative;
        if (integer < 0) {
            isNegative = true;
            integer = -integer;
        } else {
            isNegative = false;
        }

        int numChars;
        if (integer < 10L) {
            numChars = 1;
        } else if (integer < 100L) {
            numChars = 2;
        } else if (integer < 1000L) {
            numChars = 3;
        } else if (integer < 10000L) {
            numChars = 4;
        } else if (integer < 100000L) {
            numChars = 5;
        } else if (integer < 1000000L) {
            numChars = 6;
        } else if (integer < 10000000L) {
            numChars = 7;
        } else if (integer < 100000000L) {
            numChars = 8;
        } else if (integer < 1000000000L) {
            numChars = 9;
        } else if (integer < 10000000000L) {
            numChars = 10;
        } else if (integer < 100000000000L) {
            numChars = 11;
        } else if (integer < 1000000000000L) {
            numChars = 12;
        } else if (integer < 10000000000000L) {
            numChars = 13;
        } else if (integer < 100000000000000L) {
            numChars = 14;
        } else if (integer < 1000000000000000L) {
            numChars = 15;
        } else if (integer < 10000000000000000L) {
            numChars = 16;
        } else if (integer < 100000000000000000L) {
            numChars = 17;
        } else if (integer < 1000000000000000000L) {
            numChars = 18;
        } else {
            numChars = 19;
        }

        if (isNegative) {
            numChars += 1;
        }

        char[] charArray = new char[numChars];

        while (integer > 0) {
            char digit = (char) (integer % 10);
            charArray[-- numChars] = (char) (digit + 0x30);

            integer /= 10;
        }

        if (isNegative) {
            charArray[0] = '-';
        }

        return new String(charArray);
    }

    public static long parseLong(String string) throws NumberFormatException {
        return Long.parseLong(string, 10);
    }

    // See Integer.parseInt for an explanation
    public static long parseLong(String string, int radix) throws NumberFormatException {
        if (string == null) {
            throw new NumberFormatException();
        }

        if (radix < Character.MIN_RADIX || radix > Character.MAX_RADIX) {
            throw new NumberFormatException();
        }

        if (string.length() == 0) {
            throw new NumberFormatException();
        }

        long result = 0;
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

        for (; position < string.length(); position ++) {
            char thisCharChar = string.charAt(position);

            int thisChar;
            if (thisCharChar >= '0' && thisCharChar <= '9') {
                if (thisCharChar > ALL_DIGITS[radix - 1]) {
                    throw new NumberFormatException();
                } else {
                    thisChar = thisCharChar - '0';
                }
            } else if (thisCharChar >= 'a' && thisCharChar <= 'z') {
                if (thisCharChar > ALL_DIGITS[radix - 1]) {
                    throw new NumberFormatException();
                } else {
                    thisChar = (thisCharChar - 'a') + 10;
                }
            } else if (thisCharChar >= 'A' && thisCharChar <= 'Z') {
                if (thisCharChar > ALL_DIGITS_UPPER[radix - 1]) {
                    throw new NumberFormatException();
                } else {
                    thisChar = (thisCharChar - 'A') + 10;
                }
            } else {
                throw new NumberFormatException();
            }

            if (isNeg) {
                if (-result < Long.MIN_VALUE / radix) {
                    throw new NumberFormatException();
                }
            } else {
                if (result > Long.MAX_VALUE / radix) {
                    throw new NumberFormatException();
                }
            }

            if (isNeg) {
                if (-result == Long.MIN_VALUE / radix && thisChar > -(Long.MIN_VALUE % radix)) {
                    throw new NumberFormatException();
                }
            } else {
                if (result == Long.MAX_VALUE / radix && thisChar > (Long.MAX_VALUE % radix)) {
                    throw new NumberFormatException();
                }
            }

            result *= radix;
            result += thisChar;
        }

        if (isNeg) {
            return -result;
        } else {
            return result;
        }
    }

    public int compareTo(Long other) {
        if (this.value < other.value) {
            return -1;
        } else if (this.value == other.value) {
            return 0;
        } else {
            return 1;
        }
    }
}
