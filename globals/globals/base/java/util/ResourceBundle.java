package java.util;

public abstract class ResourceBundle {
    protected ResourceBundle parent;

    public final String getString(String key) {
        return (String) this.getObject(key);
    }

    public final Object getObject(String key) {
        Object result = this.handleGetObject(key);
        if (result != null) {
            return result;
        }

        // Check parent
        if (this.parent != null) {
            result = parent.getObject(key);

            if (result != null) {
                return result;
            }
        }

        throw new MissingResourceException("Can't find resource", this.getClass().getName(), key);
    }

    // Methods for subclasses to override

    protected abstract Object handleGetObject(String key);

    public abstract Enumeration<String> getKeys();
}
