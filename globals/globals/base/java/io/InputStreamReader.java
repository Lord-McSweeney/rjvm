package java.io;

import rjvm.internal.Todo;

public class InputStreamReader extends Reader {
    private InputStream stream;

    public InputStreamReader(InputStream stream) {
        super(stream);
        this.stream = stream;
    }

    public String getEncoding() {
        // TODO implement
        return "UTF8";
    }

    public void close() throws IOException {
        this.stream.close();
    }

    public int read(char[] buffer, int offset, int length) throws IOException {
        Todo.warnNotImpl("java.io.InputStreamReader.read");
        return -1;
    }
}
