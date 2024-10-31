package java.lang;

public final class StringBuilder {
    private char[] data;

    public StringBuilder() {
        this.data = new char[0];
    }

    public StringBuilder append(char[] chars) {
        char[] newData = new char[this.data.length + chars.length];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        System.arraycopy(chars, 0, newData, this.data.length, chars.length);

        this.data = newData;

        return this;
    }

    public StringBuilder append(String string) {
        char[] newData = new char[this.data.length + string.length()];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        string.getChars(0, string.length(), newData, this.data.length);

        this.data = newData;

        return this;
    }

    public StringBuilder append(int integer) {
        if (integer == -2147483648) {
            return this.append("-2147483648");
        }

        if (integer == 0) {
            // TODO append a char instead of string when it gets implemented
            return this.append("0");
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

        // TODO we can directly copy the chars into the data buffer without
        // doing (1) this allocation and (2) the allocation in append(char[])
        char[] charArray = new char[numChars];

        while (integer > 0) {
            char digit = (char) (integer % 10);
            charArray[-- numChars] = (char) (digit + 0x30);

            integer /= 10;
        }

        if (isNegative) {
            charArray[0] = '-';
        }

        return this.append(charArray);
    }

    public String toString() {
        return new String(this.data);
    }
}
