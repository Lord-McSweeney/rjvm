package java.lang;

import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintStream;

public final class System {
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

        // TODO implement
        return null;
    }

    public static String getProperty(String propName, String defaultValue) {
        if (propName == null) {
            throw new NullPointerException();
        }

        // TODO implement
        return defaultValue;
    }

    public static native void exit(int status);

    static {
        in = new FileInputStream(FileDescriptor.in);
        out = new PrintStream(new FileOutputStream(FileDescriptor.out));
        err = new PrintStream(new FileOutputStream(FileDescriptor.err));
    }
}
