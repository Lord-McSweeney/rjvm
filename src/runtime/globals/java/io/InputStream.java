package java.io;

public abstract class InputStream {
    public int available() throws IOException {
        return 0;
    }

    public abstract int read() throws IOException;

    public void close() throws IOException { }
}
