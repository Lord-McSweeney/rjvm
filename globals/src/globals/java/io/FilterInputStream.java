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
}
