package java.io;

public class ByteArrayInputStream extends InputStream {
    protected byte[] buf;
    protected int pos;

    public ByteArrayInputStream(byte[] buf) {
        this.buf = buf;
        this.pos = 0;
    }

    public int available() throws IOException {
        return this.buf.length - this.pos;
    }

    public int read() throws IOException {
        if (this.pos == this.buf.length) {
            return -1;
        } else {
            this.pos ++;

            return this.buf[this.pos - 1];
        }
    }
}
