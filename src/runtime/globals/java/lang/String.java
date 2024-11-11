package java.lang;

public final class String {
    private char[] data;

    public String(String originalString) {
        char[] data = originalString.data;
        int length = originalString.length();

        char[] copyData = new char[length];
        System.arraycopy(data, 0, copyData, 0, length);

        this.data = copyData;
    }

    public String(char[] data) {
        int length = data.length;
        char[] copyData = new char[length];
        System.arraycopy(data, 0, copyData, 0, length);

        this.data = copyData;
    }

    public static String valueOf(int integer) {
        return Integer.toString(integer);
    }

    public void getChars(int srcBegin, int srcEnd, char[] dst, int dstBegin) {
       if (
            srcBegin < 0 ||
            srcBegin > srcEnd ||
            srcEnd > this.data.length
            // Other conditions will be checked for by System.arraycopy
        ) {
            throw new IndexOutOfBoundsException();
        }

        System.arraycopy(this.data, srcBegin, dst, dstBegin, srcEnd - srcBegin);
    }

    public boolean equals(Object other) {
        if (this == other) {
            return true;
        } else if (other instanceof String) {
            String otherString = (String) other;

            if (this.length() == otherString.length()) {
                for (int i = 0; i < this.data.length; i ++) {
                    if (this.data[i] != otherString.data[i]) {
                        return false;
                    }
                }

                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    public int length() {
        return this.data.length;
    }

    public String intern() {
        // TODO implement
        return this;
    }

    public static String format(String self, Object... args) {
        // TODO implement
        return self;
    }
}
