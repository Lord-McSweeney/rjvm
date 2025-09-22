package java.nio;

public abstract class FloatBuffer extends Buffer {
    float[] data;
    int arrayOffset;

    public static FloatBuffer wrap(float[] array, int ofs, int len) {
        return new ArrayFloatBuffer(array, ofs, len);
    }

    public static FloatBuffer wrap(float[] array) {
        return FloatBuffer.wrap(array, 0, array.length);
    }

    public abstract ByteOrder order();
}

class ArrayFloatBuffer extends FloatBuffer {
    float[] data;

    ArrayFloatBuffer(float[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        this.data = array;
        this.arrayOffset = 0;

        this.position = ofs;
        this.limit = ofs + len;
    }

    public ByteOrder order() {
        // ArrayFloatBuffers always have native order
        return ByteOrder.nativeOrder();
    }
}
