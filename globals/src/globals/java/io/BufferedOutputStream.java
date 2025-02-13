package java.io;

public class BufferedOutputStream extends FilterOutputStream {
    public BufferedOutputStream(OutputStream stream) {
        super(stream);
    }
}
