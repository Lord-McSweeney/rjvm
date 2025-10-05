package java.io;

public class StringWriter extends Writer {
    public StringWriter() {
        // TODO implement
        super();
    }

    public StringWriter append(char c) {
        // TODO implement
        return this;
    }

    public void write(char[] cbuf, int off, int len) throws IOException {
        if (off < 0 || len < 0 || off + len > cbuf.length) {
            throw new IndexOutOfBoundsException();
        }

        byte[] result = new byte[len];
        for (int i = 0; i < len; i ++) {
            this.append(cbuf[off + i]);
        }
    }

    public void close() {
        // No effect
    }
}
