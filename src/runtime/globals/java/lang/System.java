package java.lang;

import java.io.FileOutputStream;
import java.io.PrintStream;

public final class System {
    public static PrintStream out = null;

    static {
        out = new PrintStream(new FileOutputStream());
    }
}
