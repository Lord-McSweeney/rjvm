package java.nio;

public abstract class CharBuffer extends Buffer {
    char[] data;
    int arrayOffset;

    public static CharBuffer wrap(char[] array, int ofs, int len) {
        return new ArrayCharBuffer(array, ofs, len);
    }

    public static CharBuffer wrap(char[] array) {
        return CharBuffer.wrap(array, 0, array.length);
    }

    public String toString() {
        return internalToString();
    }

    abstract String internalToString();
}

class ArrayCharBuffer extends CharBuffer {
    ArrayCharBuffer(char[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        this.data = array;
        this.arrayOffset = 0;

        this.position = ofs;
        this.limit = ofs + len;
    }

    String internalToString() {
        // Use chars at position..limit
        char[] newBuffer = new char[(this.limit - this.position) + 1];
        for (int i = this.position; i < this.limit; i ++) {
            newBuffer[i] = this.data[i];
        }

        return new String(newBuffer);
    }
}
