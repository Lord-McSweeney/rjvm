package java.io;

public abstract class FilterReader extends Reader {
    protected Reader in;

    protected FilterReader(Reader in) {
        super(in);
        this.in = in;
    }

    public int read() throws IOException {
        return this.in.read();
    }

    public void close() throws IOException {
        this.in.close();
    }
}
