package java.lang;

import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintStream;

final class StdoutStream extends OutputStream {
    public native void write(int b) throws IOException;
}

final class StderrStream extends OutputStream {
    public native void write(int b) throws IOException;
}

public final class System {
    public static PrintStream out = null;
    public static PrintStream err = null;

    public static native void exit(int status);

    public static native void arraycopy(Object src, int srcPos, Object dest, int destPos, int length);

    public static String getProperty(String propName) {
        // TODO implement
        return null;
    }

    static {
        out = new PrintStream(new StdoutStream());
        err = new PrintStream(new StderrStream());
    }
}
