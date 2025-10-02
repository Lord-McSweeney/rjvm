package java.io;

import java.nio.CharBuffer;
import rjvm.internal.Todo;

public abstract class Reader implements Closeable, Readable {
    protected Object lock;

    protected Reader() {
        this.lock = this;
    }

    protected Reader(Object lock) {
        if (lock == null) {
            throw new NullPointerException();
        }
        this.lock = lock;
    }

    public abstract void close() throws IOException;

    public int read(CharBuffer target) throws IOException {
        Todo.warnNotImpl("java.io.Reader.read");
        return -1;
    }

    public int read(char[] buffer) throws IOException {
        return this.read(buffer, 0, buffer.length);
    }

    public abstract int read(char[] buffer, int offset, int length) throws IOException;
}
