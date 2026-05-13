package java.util;

import java.io.InputStream;
import java.io.IOException;

public abstract class ResourceBundle {
    protected ResourceBundle parent;

    public static final ResourceBundle getBundle(String name) {
        // TODO: Choose appropriate locale, class loader, and control
        return ResourceBundle.getBundle(name, new Locale(null), ClassLoader.getSystemClassLoader(), new Control());
    }

    public static ResourceBundle getBundle(String name, Locale locale, ClassLoader loader, Control control) {
        if (loader == null || control == null) {
            throw new NullPointerException();
        }

        // TODO: First check if there's a class with the given name

        // Then we check if there's a properties file
        String propertiesName = name.replace('.', '/') + ".properties";
        InputStream readStream = loader.getResourceAsStream(propertiesName);

        if (readStream == null) {
            throw new MissingResourceException("Missing resource", null, null);
        }

        try {
            return new PropertyResourceBundle(readStream);
        } catch(IOException e) {
            // TODO what do we do here?
            throw new MissingResourceException("Invalid resource", null, null);
        }
    }

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

    public static class Control {
        protected Control() { }
    }
}
