package java.nio;

import rjvm.internal.Todo;

public abstract class ByteBuffer extends Buffer {
    ByteOrder order;

    byte[] data;
    int arrayOffset;

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

    public abstract byte get();

    public abstract byte get(int index);

    public final ByteOrder order() {
        return this.order;
    }

    public final ByteBuffer order(ByteOrder order) {
        this.order = order;
        return this;
    }

    public final byte[] array() {
        return this.data;
    }

    public final int arrayOffset() {
        return this.arrayOffset;
    }

    public abstract FloatBuffer asFloatBuffer();
}

class ArrayByteBuffer extends ByteBuffer {
    ArrayByteBuffer(byte[] array, int ofs, int len) {
        if (ofs < 0 || len < 0 || ofs + len > array.length) {
            throw new IndexOutOfBoundsException();
        }

        this.data = array;
        this.arrayOffset = 0;

        this.position = ofs;
        this.limit = ofs + len;

        // All byte buffers start off as big-endian
        this.order = ByteOrder.BIG_ENDIAN;
    }

    public FloatBuffer asFloatBuffer() {
        return new ByteBufferAsFloatBuffer(this.data, this.order);
    }

    public byte get() {
        if (this.position == this.limit) {
            throw new BufferUnderflowException();
        } else {
            byte value = this.data[this.position];
            this.position += 1;
            return value;
        }
    }

    public byte get(int index) {
        if (index >= this.limit) {
            throw new IndexOutOfBoundsException();
        } else {
            return this.data[index];
        }
    }
}

class ByteBufferAsFloatBuffer extends FloatBuffer {
    byte[] realData;
    ByteOrder order;

    // TODO: Handle limit and capacity adjustment
    ByteBufferAsFloatBuffer(byte[] data, ByteOrder order) {
        this.realData = data;
        this.order = order;
    }

    public ByteOrder order() {
        return this.order;
    }
}
