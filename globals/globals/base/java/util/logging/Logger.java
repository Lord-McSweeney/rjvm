package java.util.logging;

public class Logger {
    protected Logger(String name, String resourceBundleName) {
    }

    public void entering(String sourceClass, String sourceMethod) {
        // TODO implement
    }

    public void entering(String sourceClass, String sourceMethod, Object param1) {
        // TODO implement
    }

    public void entering(String sourceClass, String sourceMethod, Object[] params) {
        // TODO implement
    }

    public void log(Level level, String msg) {
        // TODO implement
    }

    public void log(Level level, String msg, Object param1) {
        // TODO implement
    }

    public void log(Level level, String msg, Object[] params) {
        // TODO implement
    }

    public void exiting(String sourceClass, String sourceMethod) {
        // TODO implement
    }

    public void exiting(String sourceClass, String sourceMethod, Object result) {
        // TODO implement
    }

    public static Logger getLogger(String name) {
        // TODO implement properly
        return new Logger(name, null);
    }
}
