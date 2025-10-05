package java.io;

public abstract class Writer implements Closeable {
    protected Writer() { }

    public void write(char[] cbuf) throws IOException {
        this.write(cbuf, 0, cbuf.length);
    }

    public abstract void write(char[] cbuf, int off, int len) throws IOException;

    public abstract void close() throws IOException;
}
