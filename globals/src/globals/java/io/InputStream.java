package java.io;

public abstract class InputStream implements Closeable {
    public int available() throws IOException {
        return 0;
    }

    public abstract int read() throws IOException;

    public void close() throws IOException { }
}
