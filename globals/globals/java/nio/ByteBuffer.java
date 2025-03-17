package java.nio;

public abstract class ByteBuffer extends Buffer {
    public static ByteBuffer wrap(byte[] array, int ofs, int len) {
        return new ArrayByteBuffer(array, ofs, len);
    }

    public static ByteBuffer wrap(byte[] array) {
        return ByteBuffer.wrap(array, 0, array.length);
    }
}

class ArrayByteBuffer extends ByteBuffer {
    byte[] bytes;

    ArrayByteBuffer(byte[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        byte[] bytes = new byte[len];
        for (int i = ofs; i < ofs + len; i ++) {
            bytes[i] = array[i];
        }
        this.bytes = bytes;
    }
}
