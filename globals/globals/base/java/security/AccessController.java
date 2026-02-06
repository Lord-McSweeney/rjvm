package java.security;

public final class AccessController {
    public static <T> T doPrivileged(PrivilegedAction<T> action) {
        // TODO implement privileges
        return action.run();
    }
}
