package java.lang;

import java.io.PrintStream;

public class Throwable {
    // NOTE field ordering is important
    private String message;

    private StackTraceElement[] stackTrace;

    // If cause is equal to self, then it's not-yet-set
    private Throwable cause;

    // Constructors
    public Throwable() {
        this.fillInStackTrace();

        // Make cause not-yet-set
        this.cause = this;
    }

    public Throwable(String message) {
        this.fillInStackTrace();
        this.message = message;

        // Make cause not-yet-set
        this.cause = this;
    }

    public Throwable(String message, Throwable cause) {
        this.fillInStackTrace();
        this.message = message;

        this.cause = cause;
    }

    public Throwable(Throwable cause) {
        this.fillInStackTrace();

        if (cause != null) {
            this.message = cause.toString();
        }

        this.cause = cause;
    }

    public Throwable fillInStackTrace() {
        this.stackTrace = Throwable.internalFillInStackTrace();
        return this;
    }

    private static native StackTraceElement[] internalFillInStackTrace();

    public synchronized Throwable getCause() {
        if (this.cause == this) {
            return null;
        } else {
            return this.cause;
        }
    }

    public synchronized Throwable initCause(Throwable cause) {
        if (this.cause != this) {
            // Cause was already set
            throw new IllegalStateException();
        } else if (cause == this) {
            // Cannot have cause set to self
            throw new IllegalArgumentException();
        } else {
            this.cause = cause;
            return this;
        }
    }

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

        // TODO guard against recursion
        if (this.cause != this) {
            s.print("Caused by: ");
            this.cause.printStackTrace(s);
        }
    }
}
