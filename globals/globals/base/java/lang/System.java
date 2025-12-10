package java.lang;

import rjvm.internal.Todo;
import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintStream;
import java.util.Properties;

public final class System {
    private static Properties properties = null;

    public static InputStream in = null;
    public static PrintStream out = null;
    public static PrintStream err = null;

    private System() { }

    // Java code to initialize the VM. This method is called by the native
    // function `Context::load_builtins`.
    private static void initSystemClass() {
        ClassLoader.getSystemClassLoader();

        properties = new Properties();
        // TODO initialize these natively
        properties.setProperty("file.separator", "/");
        properties.setProperty("path.separator", ":");
        properties.setProperty("line.separator", "\n");

        in = new FileInputStream(FileDescriptor.in);
        // Enable `autoFlush` on these streams. If we don't, reading from stdin
        // after writing to stdout without writing a newline to flush the
        // terminal buffer won't work
        out = new PrintStream(new FileOutputStream(FileDescriptor.out), true);
        err = new PrintStream(new FileOutputStream(FileDescriptor.err), true);
    }

    // Stream code
    public static void setIn(InputStream in) {
        System.in = in;
    }

    public static void setOut(PrintStream out) {
        System.out = out;
    }

    public static void setErr(PrintStream err) {
        System.err = err;
    }

    // arraycopy
    public static native void arraycopy(Object src, int srcPos, Object dest, int destPos, int length);

    // Properties code
    public static String getProperty(String propName) {
        return System.getProperty(propName, null);
    }

    public static String getProperty(String propName, String defaultValue) {
        if (propName == null) {
            throw new NullPointerException();
        }

        if (propName == "") {
            throw new IllegalArgumentException();
        }

        return System.properties.getProperty(propName, defaultValue);
    }

    public static String setProperty(String propName, String newValue) {
        if (propName == null || newValue == null) {
            throw new NullPointerException();
        }

        if (propName == "") {
            throw new IllegalArgumentException();
        }

        return (String) System.properties.setProperty(propName, newValue);
    }

    // Misc native functions
    public static native long currentTimeMillis();

    public static long nanoTime() {
        // :p
        return System.currentTimeMillis() * 1000000;
    }

    public static native int identityHashCode(Object x);

    public static void gc() {
        Runtime.getRuntime().gc();
    }

    public static void exit(int status) {
        Runtime.getRuntime().exit(status);
    }
}
