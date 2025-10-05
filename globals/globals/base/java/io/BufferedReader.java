package java.io;

import rjvm.internal.Todo;

// TODO implement buffering
public class BufferedReader extends Reader {
    private Reader reader;

    public BufferedReader(Reader reader) {
        this.reader = reader;
    }

    public String readLine() throws IOException {
        Todo.warnNotImpl("java.io.BufferedReader.readLine");

        return "";
    }

    public void close() throws IOException {
        this.reader.close();
    }

    public int read(char[] cbuf, int off, int len) throws IOException {
        return this.reader.read(cbuf, off, len);
    }
}
