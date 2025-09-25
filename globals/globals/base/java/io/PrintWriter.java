package java.io;

import rjvm.internal.Todo;

public class PrintWriter extends Writer {
    public PrintWriter(Writer writer) {
        // TODO implement
        super();
    }

    public PrintWriter(Writer writer, boolean autoFlush) {
        // TODO implement
        super();
    }

    public PrintWriter(OutputStream out) {
        // TODO implement
        super();
    }

    public PrintWriter(OutputStream out, boolean autoFlush) {
        // TODO implement
        super();
    }

    public void print(String data) {
        Todo.warnNotImpl("java.io.PrintWriter.print");
    }

    public void println() {
        Todo.warnNotImpl("java.io.PrintWriter.println");
    }

    public void println(String data) {
        Todo.warnNotImpl("java.io.PrintWriter.println");
    }

    public void write(String data) {
        Todo.warnNotImpl("java.io.PrintWriter.write");
    }

    public void close() {
        Todo.warnNotImpl("java.io.PrintWriter.close");
    }
}
