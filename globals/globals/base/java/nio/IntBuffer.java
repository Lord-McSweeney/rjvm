package java.nio;

public abstract class IntBuffer extends Buffer {
    int[] data;
    int arrayOffset;

    public static IntBuffer wrap(int[] array, int ofs, int len) {
        return new ArrayIntBuffer(array, ofs, len);
    }

    public static IntBuffer wrap(int[] array) {
        return IntBuffer.wrap(array, 0, array.length);
    }

    public abstract ByteOrder order();
}

class ArrayIntBuffer extends IntBuffer {
    int[] data;

    ArrayIntBuffer(int[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        this.data = array;
        this.arrayOffset = 0;

        this.position = ofs;
        this.limit = ofs + len;
    }

    public ByteOrder order() {
        // ArrayIntBuffers always have native order
        return ByteOrder.nativeOrder();
    }
}
