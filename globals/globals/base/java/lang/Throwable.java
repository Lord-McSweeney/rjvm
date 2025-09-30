package java.lang;

import java.io.PrintStream;

public class Throwable {
    private String message;

    // Should technically be an array of stack trace elements
    // NOTE field ordering is important
    private String stackTrace;

    public Throwable() {
        this.fillInStackTrace();
    }

    public Throwable(String message) {
        this.fillInStackTrace();
        this.message = message;
    }

    public Throwable fillInStackTrace() {
        this.stackTrace = this.internalFillInStackTrace();
        return this;
    }

    private native String internalFillInStackTrace();

    public String getMessage() {
        return this.message;
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

        s.print(this.stackTrace);
    }
}
