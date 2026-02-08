package java.util;

public class MissingResourceException extends RuntimeException {
    public MissingResourceException(String message, String className, String key) {
        super(message);
    }
}
