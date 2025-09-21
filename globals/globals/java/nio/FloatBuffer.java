package java.nio;

public abstract class FloatBuffer extends Buffer {
    public static FloatBuffer wrap(float[] array, int ofs, int len) {
        return new ArrayFloatBuffer(array, ofs, len);
    }

    public static FloatBuffer wrap(float[] array) {
        return FloatBuffer.wrap(array, 0, array.length);
    }
}

class ArrayFloatBuffer extends FloatBuffer {
    float[] data;

    ArrayFloatBuffer(float[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        float[] data = new float[len];
        for (int i = ofs; i < ofs + len; i ++) {
            data[i] = array[i];
        }
        this.data = data;
    }
}
