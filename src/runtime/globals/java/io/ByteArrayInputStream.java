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
        this.pos ++;
        if (this.pos == this.buf.length) {
            // Make sure to revert position so a call to available()
            // doesn't return a negative number
            this.pos --;

            return -1;
        } else {
            return this.buf[this.pos - 1];
        }
    }
}
