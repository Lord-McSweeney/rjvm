package java.lang;

public final class Long extends Number implements Comparable<Long> {
    public static Class<Long> TYPE = (Class<Long>) Class.getPrimitiveClass(Class.PRIM_LONG);

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

    public String toString() {
        return Long.toString(this.value);
    }

    public long longValue() {
        return this.value;
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
