package java.io;

public class OutputStreamWriter extends Writer {
    private OutputStream stream;

    public OutputStreamWriter(OutputStream stream) {
        this.stream = stream;
    }

    public void write(char[] cbuf, int off, int len) throws IOException {
        if (off < 0 || len < 0 || off + len > cbuf.length) {
            throw new IndexOutOfBoundsException();
        }

        // TODO implement decoding
        byte[] result = new byte[len];
        for (int i = 0; i < len; i ++) {
            result[i] = (byte) cbuf[off + i];
        }

        this.stream.write(result);
    }

    public void close() throws IOException {
        this.stream.flush();
        this.stream.close();
    }
}
