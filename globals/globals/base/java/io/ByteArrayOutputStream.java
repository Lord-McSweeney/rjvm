package java.io;

public class ByteArrayOutputStream extends OutputStream {
    protected byte[] buf;
    protected int count;

    public ByteArrayOutputStream() {
        this(32);
    }

    public ByteArrayOutputStream(int capacity) {
        if (capacity < 0) {
            throw new IllegalArgumentException();
        }

        this.buf = new byte[capacity];
        this.count = 0;
    }

    public synchronized void write(int b) {
        byte bByte = (byte) b;
        if (this.count == this.buf.length) {
            byte[] newBuf = new byte[this.buf.length * 2];
            System.arraycopy(this.buf, 0, newBuf, 0, this.buf.length);
            this.buf = newBuf;
        }
        this.buf[this.count ++] = bByte;
    }

    public synchronized void write(byte[] b, int off, int len) {
        if (off < 0 || len < 0 || off + len > b.length) {
            throw new IndexOutOfBoundsException();
        }

        if (this.count + len > this.buf.length) {
            byte[] newBuf = new byte[this.buf.length * 2 + len];
            System.arraycopy(this.buf, 0, newBuf, 0, this.buf.length);
            this.buf = newBuf;
        }

        for (int i = 0; i < len; i ++) {
            this.buf[this.count ++] = b[off + i];
        }
    }

    public synchronized void writeTo(OutputStream stream) throws IOException {
        stream.write(this.buf, 0, this.count);
    }

    public synchronized int size() {
        return this.count;
    }

    public synchronized byte[] toByteArray() {
        byte[] newBuf = new byte[this.count];
        System.arraycopy(this.buf, 0, newBuf, 0, this.count);
        return newBuf;
    }

    public synchronized void reset() {
        this.count = 0;
    }

    public void close() throws IOException {
        // Does nothing
    }
}
