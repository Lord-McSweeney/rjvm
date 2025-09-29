package java.io;

public abstract class InputStream implements Closeable {
    public int available() throws IOException {
        return 0;
    }

    public abstract int read() throws IOException;

    public int read(byte buffer[]) throws IOException {
        return this.read(buffer, 0, buffer.length);
    }

    public int read(byte buffer[], int ofs, int len) throws IOException {
        if (buffer == null) {
            throw new NullPointerException();
        } else if (ofs < 0 || len < 0 || ofs + len > buffer.length) {
            throw new IndexOutOfBoundsException();
        }

        int count = 0;
        while (count < len) {
            int read = this.read();
            if (read == -1) {
                break;
            }

            buffer[ofs + count] = (byte) read;

            count += 1;
        }

        return count;
    }

    public void close() throws IOException { }
}
