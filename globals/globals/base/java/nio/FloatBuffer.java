package java.nio;

public abstract class FloatBuffer extends Buffer {
    public static FloatBuffer wrap(float[] array, int ofs, int len) {
        return new ArrayFloatBuffer(array, ofs, len);
    }

    public static FloatBuffer wrap(float[] array) {
        return FloatBuffer.wrap(array, 0, array.length);
    }

    public abstract FloatBuffer put(float f);

    public FloatBuffer put(float[] array, int ofs, int len) {
        // TODO bounds checks

        for (int i = 0; i < len; i ++) {
            this.put(array[ofs + i]);
        }

        return this;
    }

    public final FloatBuffer put(float[] array) {
        return this.put(array, 0, array.length);
    }

    public abstract ByteOrder order();
}

class ArrayFloatBuffer extends FloatBuffer {
    float[] data;
    int arrayOffset;

    ArrayFloatBuffer(float[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        this.data = array;
        this.arrayOffset = 0;

        this.position = ofs;
        this.limit = ofs + len;
    }

    public FloatBuffer put(float f) {
        int position = this.checkGetNextPosition();
        this.data[position] = f;
        return this;
    }

    public ByteOrder order() {
        // ArrayFloatBuffers always have native order
        return ByteOrder.nativeOrder();
    }
}
