package java.lang;

public final class Integer extends Number implements Comparable<Integer> {
    private int value;

    public Integer(int value) {
        this.value = value;
    }

    public String toString() {
        return Integer.toString(this.value);
    }

    public static Integer valueOf(int integer) {
        return new Integer(integer);
    }

    public static String toString(int integer) {
        if (integer == -2147483648) {
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
            charArray[-- numChars] = (char) (digit + 0x30);

            integer /= 10;
        }

        if (isNegative) {
            charArray[0] = '-';
        }

        return new String(charArray);
    }

    public static int parseInt(String string) {
        // TODO implement
        return 0;
    }
}
