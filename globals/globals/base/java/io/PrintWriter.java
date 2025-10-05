package java.io;

import rjvm.internal.Todo;

public class PrintWriter extends Writer {
    private Writer writer;
    private boolean autoFlush;

    public PrintWriter(Writer writer) {
        this(writer, false);
    }

    public PrintWriter(Writer writer, boolean autoFlush) {
        this.writer = writer;
        this.autoFlush = autoFlush;
    }

    public PrintWriter(OutputStream out) {
        this(out, false);
    }

    public PrintWriter(OutputStream out, boolean autoFlush) {
        this.writer = new OutputStreamWriter(out);
        this.autoFlush = autoFlush;
    }

    public void write(char[] cbuf, int off, int len) throws IOException {
        this.writer.write(cbuf, off, len);
    }

    public void print(String data) {
        Todo.warnNotImpl("java.io.PrintWriter.print");
    }

    public void println() {
        try {
            this.write(new char[]{'\n'});
        } catch (IOException e) {
            // TODO report errors
        }
    }

    public void println(String data) {
        char[] chars = new char[data.length()];
        data.getChars(0, data.length(), chars, 0);

        try {
            this.write(chars);
        } catch (IOException e) {
            // TODO report errors
        }
        this.println();
    }

    public void write(String data) {
        Todo.warnNotImpl("java.io.PrintWriter.write");
    }

    public void close() {
        Todo.warnNotImpl("java.io.PrintWriter.close");
    }
}
