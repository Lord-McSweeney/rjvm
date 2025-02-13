package java.io;

import java.lang.IndexOutOfBoundsException;
import java.lang.NullPointerException;

public abstract class OutputStream {
    public abstract void write(int b) throws IOException;

    public void write(byte buffer[], int ofs, int len) throws IOException {
        if (buffer == null) {
            throw new NullPointerException();
        }

        if (ofs < 0 || len < 0 || ofs + len > buffer.length || ofs + len < 0) {
            throw new IndexOutOfBoundsException();
        }
        
        for (int i = ofs; i < ofs + len; i ++) {
            this.write(buffer[i]);
        }
    }

    public void write(byte buffer[]) throws IOException {
        write(buffer, 0, buffer.length);
    }

    public void flush() throws IOException { }
}
