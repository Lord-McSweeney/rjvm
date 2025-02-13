package java.io;

public class FilterInputStream extends InputStream {
    protected InputStream in;

    public FilterInputStream(InputStream filteredStream) {
        this.in = filteredStream;
    }

    public int read() throws IOException {
        return -1;
    }
}
