package java.io;

public class FilterOutputStream extends OutputStream {
    protected OutputStream out;

    public FilterOutputStream(OutputStream filteredStream) {
        this.out = filteredStream;
    }

    public void write(int b) throws IOException {
        this.out.write(b);
    }

    public void write(byte buffer[], int ofs, int len) throws IOException {
        if (buffer == null) {
            throw new NullPointerException();
        }

        if (ofs < 0 || len < 0 || ofs + len > buffer.length || ofs + len < 0) {
            throw new IndexOutOfBoundsException();
        }

        for (int i = ofs; i < ofs + len; i ++) {
            write(buffer[i]);
        }
    }

    public void write(byte buffer[]) throws IOException {
        this.out.write(buffer);
    }

    public void flush() throws IOException {
        this.out.flush();
    }
}
