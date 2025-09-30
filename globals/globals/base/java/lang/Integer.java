package java.lang;

import rjvm.internal.Todo;

public final class Integer extends Number implements Comparable<Integer> {
    public static Class<Integer> TYPE = (Class<Integer>) Class.getPrimitiveClass(Class.PRIM_INT);

    public static final int MIN_VALUE = -2147483648;
    public static final int MAX_VALUE = 2147483647;

    private static final char[] ALL_DIGITS = new char[]{
        '0', '1', '2', '3', '4', '5', '6', '7', '8',
        '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
        'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    };

    private int value;

    public Integer(int value) {
        this.value = value;
    }

    public boolean equals(Object obj) {
        if (obj instanceof Integer) {
            Integer other = (Integer) obj;
            return this.value == other.value;
        } else {
            return false;
        }
    }

    public String toString() {
        return Integer.toString(this.value);
    }

    public int intValue() {
        return this.value;
    }

    public static Integer valueOf(int integer) {
        return new Integer(integer);
    }

    public static String toString(int integer) {
        // Special-case for radix 10 because most code will use this
        if (integer == Integer.MIN_VALUE) {
            return "-2147483648";
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
        if (integer < 10) {
            numChars = 1;
        } else if (integer < 100) {
            numChars = 2;
        } else if (integer < 1000) {
            numChars = 3;
        } else if (integer < 10000) {
            numChars = 4;
        } else if (integer < 100000) {
            numChars = 5;
        } else if (integer < 1000000) {
            numChars = 6;
        } else if (integer < 10000000) {
            numChars = 7;
        } else if (integer < 100000000) {
            numChars = 8;
        } else if (integer < 1000000000) {
            numChars = 9;
        } else {
            numChars = 10;
        }

        if (isNegative) {
            numChars += 1;
        }

        char[] charArray = new char[numChars];

        while (integer > 0) {
            char digit = (char) (integer % 10);
            charArray[-- numChars] = (char) (digit + '0');

            integer /= 10;
        }

        if (isNegative) {
            charArray[0] = '-';
        }

        return new String(charArray);
    }

    public static String toString(int integer, int radix) {
        if (integer == 0) {
            return "0";
        }

        if (radix < Character.MIN_RADIX || radix > Character.MAX_RADIX) {
            radix = 10;
        }

        char[] result = new char[33];
        int position = 32;
        boolean isNeg = true;

        // Can't do it the other way around because of -MIN_VALUE
        if (integer >= 0) {
            integer = -integer;
            isNeg = false;
        }

        while (integer < 0) {
            result[position --] = ALL_DIGITS[-(integer % radix)];

            integer /= radix;
        }

        if (isNeg) {
            result[position --] = '-';
        }

        return new String(result, position + 1, 32 - position);
    }

    public static String toHexString(int integer) {
        if (integer == 0) {
            return "0";
        }

        char[] result = new char[8];
        int position = 7;

        while (integer != 0) {
            result[position --] = ALL_DIGITS[integer & 0xF];
            integer >>>= 4;
        }

        return new String(result, position + 1, 7 - position);
    }

    public static int parseInt(String string) throws NumberFormatException {
        if (string.length() == 0) {
            throw new NumberFormatException();
        }

        int result = 0;
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
            char thisChar = string.charAt(position);
            if (thisChar < '0' || thisChar > '9') {
                throw new NumberFormatException();
            }

            // Current result is `214748365` or greater
            if (result > Integer.MAX_VALUE / 10) {
                throw new NumberFormatException();
            }

            if (isNeg) {
                // Current result is `-214748364`, and the current char is > "8"
                if (result == Integer.MAX_VALUE / 10 && thisChar > '8') {
                    throw new NumberFormatException();
                }
            } else {
                // Current result is `214748364`, and the current char is > "7"
                if (result == Integer.MAX_VALUE / 10 && thisChar > '7') {
                    throw new NumberFormatException();
                }
            }

            result *= 10;
            result += thisChar - '0';
        }

        if (isNeg) {
            return -result;
        } else {
            return result;
        }
    }

    public int compareTo(Integer other) {
        if (this.value < other.value) {
            return -1;
        } else if (this.value == other.value) {
            return 0;
        } else {
            return 1;
        }
    }
}
