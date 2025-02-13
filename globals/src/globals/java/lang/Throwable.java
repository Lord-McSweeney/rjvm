package java.lang;

public class Throwable {
    private String message;

    public Throwable() {
        // Would normally initialize a stack trace, but we don't support that
    }

    public Throwable(String message) {
        this.message = message;
    }
}
