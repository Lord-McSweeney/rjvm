package java.lang;

import java.io.PrintStream;

public class Throwable {
    private String message;

    // NOTE field ordering is important
    private StackTraceElement[] stackTrace;

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

    private native StackTraceElement[] internalFillInStackTrace();

    public String getMessage() {
        return this.message;
    }

    public StackTraceElement[] getStackTrace() {
        return this.stackTrace;
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

        for (int i = 0; i < this.stackTrace.length; i ++) {
            StringBuilder builder = new StringBuilder();
            builder.append("\tat ");
            builder.append(this.stackTrace[i]);

            s.println(builder);
        }
    }
}
