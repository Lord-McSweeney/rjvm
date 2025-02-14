package java.lang;

import java.nio.charset.Charset;

public final class String {
    private char[] data;

    // Constructors

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

    public String(char[] data, int start, int length) {
        if (start < 0 || length < 0 || start + length > data.length) {
            throw new StringIndexOutOfBoundsException();
        }

        char[] copyData = new char[length];
        System.arraycopy(data, start, copyData, 0, length);

        this.data = copyData;
    }

    // Overriden from Object

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

    public int hashCode() {
        // TODO implement better
        return this.length();
    }

    public String toString() {
        return this;
    }

    // Own functions

    public char charAt(int index) {
        if (index < 0 || index >= this.data.length) {
            throw new StringIndexOutOfBoundsException();
        }

        return this.data[index];
    }

    public int codePointAt(int index) {
        // TODO return correct result when char at index is a part of a surrogate pair

        if (index < 0 || index >= this.data.length) {
            throw new StringIndexOutOfBoundsException();
        }

        return (int) this.data[index];
    }

    public byte[] getBytes() {
        // TODO implement proper decoding
        return Charset.stringToUtf8(this);
    }

    public byte[] getBytes(String charsetName) {
        // TODO implement proper decoding
        return Charset.stringToUtf8(this);
    }

    public void getChars(int srcBegin, int srcEnd, char[] dst, int dstBegin) {
        if (
            srcBegin < 0 ||
            srcBegin > srcEnd ||
            srcEnd > this.data.length
            // Other conditions will be checked for by System.arraycopy
        ) {
            throw new StringIndexOutOfBoundsException();
        }

        System.arraycopy(this.data, srcBegin, dst, dstBegin, srcEnd - srcBegin);
    }

    public String trim() {
        int start = 0;
        int end = this.data.length;

        while (this.data[start] <= ' ') {
            start += 1;

            if (start == this.data.length) {
                return "";
            }
        }

        while (this.data[end - 1] <= ' ') {
            end -= 1;

            if (end == 0) {
                return "";
            }
        }

        // An allocation could be skipped here, but it shouldn't be too important
        char[] newBuffer = new char[end - start];
        this.getChars(start, end, newBuffer, 0);
        return new String(newBuffer);
    }

    public boolean endsWith(String suffix) {
        int thisLength = this.length();
        int suffixLength = suffix.length();

        if (suffixLength > thisLength) {
            return false;
        }

        for (int i = 0; i < suffixLength; i ++) {
            if (this.data[thisLength - i - 1] != suffix.data[suffixLength - i - 1]) {
                return false;
            }
        }

        return true;
    }

    public int indexOf(int search) {
        return this.indexOf(search, 0);
    }

    public int indexOf(int search, int fromIndex) {
        if (fromIndex < 0) {
            fromIndex = 0;
        }

        // TODO support code points
        char searchChar = (char) search;
        for (int i = fromIndex; i < this.data.length; i ++) {
            if (this.data[i] == searchChar) {
                return i;
            }
        }

        return -1;
    }

    public int lastIndexOf(int search) {
        // TODO support code points
        char searchChar = (char) search;
        for (int i = this.data.length - 1; i >= 0; i --) {
            if (this.data[i] == searchChar) {
                return i;
            }
        }

        return -1;
    }

    public int indexOf(String search) {
        return this.indexOf(search, 0);
    }

    public int indexOf(String search, int fromIndex) {
        if (search.length() > this.length()) {
            return -1;
        }

        if (fromIndex < 0) {
            fromIndex = 0;
        }

        for (int i = fromIndex; i < this.data.length; i ++) {
            boolean failedToMatch = false;
            for (int j = 0; j < search.length(); j ++) {
                if (this.data[i + j] != search.data[j]) {
                    failedToMatch = true;
                    break;
                }
            }

            if (!failedToMatch) {
                return i;
            }
        }

        return -1;
    }

    public String substring(int start) {
        return this.substring(start, this.data.length);
    }

    public String substring(int start, int end) {
       if (
            start < 0 ||
            start > end ||
            end > this.data.length
        ) {
            throw new StringIndexOutOfBoundsException();
        }

        // TODO dependent strings
        return new String(this.data, start, end - start);
    }

    public int length() {
        return this.data.length;
    }

    public String intern() {
        // TODO implement
        return this;
    }

    // Static functions

    public static String format(String self, Object... args) {
        // TODO implement
        return self;
    }

    public static String valueOf(int integer) {
        return Integer.toString(integer);
    }
}
