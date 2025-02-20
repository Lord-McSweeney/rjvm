package java.lang;

import java.io.PrintStream;

public class Throwable {
    private String message;

    public Throwable() {
        // Would normally initialize a stack trace, but we don't support that
    }

    public Throwable(String message) {
        this.message = message;
    }

    public String toString() {
        String className = this.getClass().getName();

        if (this.message == null) {
            return className;
        } else {
            return className + ": " + this.message;
        }
    }

    public void printStackTrace() {
        this.printStackTrace(System.err);
    }

    public void printStackTrace(PrintStream s) {
        s.println(this);

        // TODO print stack trace
    }
}
