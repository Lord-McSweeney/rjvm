package java.nio;

public abstract class CharBuffer extends Buffer {
    public static CharBuffer wrap(char[] array, int offset, int length) {
        // TODO implement
        return null;
    }

    public static CharBuffer wrap(char[] array) {
        return CharBuffer.wrap(array, 0, array.length);
    }
}
