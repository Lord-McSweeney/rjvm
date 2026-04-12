package java.lang;

public class AssertionError extends Error {
    public AssertionError() {
        super();
    }

    public AssertionError(int message) {
        super(String.valueOf(message));
    }

    public AssertionError(Object message) {
        super(String.valueOf(message));
    }
}
