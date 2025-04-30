package java.nio;

public abstract class CharBuffer extends Buffer {
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
    char[] data;

    ArrayCharBuffer(char[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        char[] data = new char[len];
        for (int i = ofs; i < ofs + len; i ++) {
            data[i] = array[i];
        }
        this.data = data;
    }

    String internalToString() {
        return new String(this.data);
    }
}
