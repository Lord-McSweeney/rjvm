package java.io;

public class FilterInputStream extends InputStream {
    protected InputStream in;

    public FilterInputStream(InputStream filteredStream) {
        this.in = filteredStream;
    }

    public int available() throws IOException {
        return this.in.available();
    }

    public int read() throws IOException {
        return this.in.read();
    }

    public int read(byte buffer[]) throws IOException {
        return this.read(buffer, 0, buffer.length);
    }

    public int read(byte buffer[], int offset, int length) throws IOException {
        return this.in.read(buffer, offset, length);
    }

    public void close() throws IOException {
        this.in.close();
    }
}
