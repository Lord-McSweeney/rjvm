package java.nio;

public abstract class ByteBuffer extends Buffer {
    public static ByteBuffer wrap(byte[] array, int offset, int length) {
        // TODO implement
        return null;
    }

    public static ByteBuffer wrap(byte[] array) {
        return ByteBuffer.wrap(array, 0, array.length);
    }
}
