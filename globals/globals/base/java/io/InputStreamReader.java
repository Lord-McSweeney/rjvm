package java.io;

import rjvm.internal.Todo;

public class InputStreamReader extends Reader {
    public void close() throws IOException {
        Todo.warnNotImpl("java.io.InputStreamReader.close");
    }

    public int read(char[] buffer, int offset, int length) throws IOException {
        Todo.warnNotImpl("java.io.InputStreamReader.read");
        return -1;
    }
}
