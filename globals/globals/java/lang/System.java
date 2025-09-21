package java.lang;

import rjvm.internal.Todo;
import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintStream;

public final class System {
    private System() { }

    public static InputStream in = null;
    public static PrintStream out = null;
    public static PrintStream err = null;

    public static void setIn(InputStream in) {
        System.in = in;
    }

    public static void setOut(PrintStream out) {
        System.out = out;
    }

    public static void setErr(PrintStream err) {
        System.err = err;
    }

    public static native void arraycopy(Object src, int srcPos, Object dest, int destPos, int length);

    public static String getProperty(String propName) {
        if (propName == null) {
            throw new NullPointerException();
        }

        Todo.warnNotImpl("java.lang.System.getProperty");

        return null;
    }

    public static String getProperty(String propName, String defaultValue) {
        if (propName == null) {
            throw new NullPointerException();
        }

        Todo.warnPartialImpl("java.lang.System.getProperty");

        return defaultValue;
    }

    public static String setProperty(String propName, String newValue) {
        if (propName == null || newValue == null) {
            throw new NullPointerException();
        }

        Todo.warnNotImpl("java.lang.System.setProperty");

        return null;
    }

    public static native long nanoTime();

    public static void exit(int status) {
        Runtime.getRuntime().exit(status);
    }

    static {
        in = new FileInputStream(FileDescriptor.in);
        out = new PrintStream(new FileOutputStream(FileDescriptor.out));
        err = new PrintStream(new FileOutputStream(FileDescriptor.err));
    }
}
