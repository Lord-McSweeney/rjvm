package java.io;

public class FilterOutputStream extends OutputStream {
    private OutputStream stream;

    public FilterOutputStream(OutputStream filteredStream) {
        this.stream = filteredStream;
    }

    public void write(int b) throws IOException {
        this.stream.write(b);
    }

    public void write(byte buffer[], int ofs, int len) throws IOException {
        this.stream.write(buffer, ofs, len);
    }

    public void write(byte buffer[]) throws IOException {
        this.stream.write(buffer);
    }

    public void flush() throws IOException {
        this.stream.flush();
    }
}
