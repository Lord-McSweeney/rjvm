package java.io;

public abstract class Writer implements Closeable {
    protected Writer() { }

    public abstract void close() throws IOException;
}
