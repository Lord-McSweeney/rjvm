package java.lang;

public class ExceptionInInitializerError extends LinkageError {
    public ExceptionInInitializerError() {
        this(null);
    }

    public ExceptionInInitializerError(Throwable throwable) {
        this.initCause(throwable);
    }

    public Throwable getException() {
        return super.getCause();
    }

    public Throwable getCause() {
        return super.getCause();
    }
}
