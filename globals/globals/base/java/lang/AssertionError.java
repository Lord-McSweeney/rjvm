package java.lang;

public class AssertionError extends Error {
    public AssertionError() {
        super();
    }

    public AssertionError(Object message) {
        super(String.valueOf(message));
    }
}
