package java.io;

import rjvm.internal.Todo;

public class PrintWriter extends Writer {
    // This needs to be called `out` because subclasses can access it
    protected Writer out;

    private boolean autoFlush;

    public PrintWriter(Writer writer) {
        this(writer, false);
    }

    public PrintWriter(Writer writer, boolean autoFlush) {
        this.out = writer;
        this.autoFlush = autoFlush;
    }

    public PrintWriter(OutputStream out) {
        this(out, false);
    }

    public PrintWriter(OutputStream out, boolean autoFlush) {
        this.out = new OutputStreamWriter(out);
        this.autoFlush = autoFlush;
    }

    public void write(char[] cbuf, int off, int len) throws IOException {
        this.out.write(cbuf, off, len);
    }

    public void print(char c) {
        try {
            this.write(new char[]{c});
        } catch (IOException e) {
            // TODO report errors
        }
    }

    public void print(String data) {
        this.write(data);
    }

    public void println() {
        this.print('\n');
    }

    public void println(String data) {
        this.print(data);
        this.println();
    }

    public void write(String data) {
        char[] chars = new char[data.length()];
        data.getChars(0, data.length(), chars, 0);

        try {
            this.write(chars);
        } catch (IOException e) {
            // TODO report errors
        }
    }

    public void close() {
        if (this.out == null) {
            return;
        }

        try {
            this.out.close();
            this.out = null;
        } catch(IOException e) {
            // TODO report errors
        }
    }
}
