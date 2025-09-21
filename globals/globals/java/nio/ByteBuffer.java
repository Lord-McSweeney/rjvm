package java.nio;

import rjvm.internal.Todo;

public abstract class ByteBuffer extends Buffer {
    private ByteOrder order;

    public static ByteBuffer wrap(byte[] array, int ofs, int len) {
        return new ArrayByteBuffer(array, ofs, len);
    }

    public static ByteBuffer wrap(byte[] array) {
        return ByteBuffer.wrap(array, 0, array.length);
    }

    public static ByteBuffer allocateDirect(int capacity) {
        byte[] array = new byte[capacity];
        return ByteBuffer.wrap(array);
    }

    public final ByteOrder order() {
        return this.order;
    }

    public final ByteBuffer order(ByteOrder order) {
        this.order = order;
        return this;
    }

    public abstract FloatBuffer asFloatBuffer();
}

class ArrayByteBuffer extends ByteBuffer {
    byte[] data;

    ArrayByteBuffer(byte[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        byte[] data = new byte[len];
        for (int i = ofs; i < ofs + len; i ++) {
            data[i] = array[i];
        }
        this.data = data;
    }

    public FloatBuffer asFloatBuffer() {
        Todo.warnNotImpl("ByteBuffer.asFloatBuffer");
        return null;
    }
}
