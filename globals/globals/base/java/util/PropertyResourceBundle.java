package java.util;

import java.io.InputStream;
import java.io.IOException;

public class PropertyResourceBundle extends ResourceBundle {
    private Properties properties;

    public PropertyResourceBundle(InputStream stream) throws IOException {
        Properties properties = new Properties();
        properties.load(stream);

        this.properties = properties;
    }

    protected Object handleGetObject(String key) {
        if (key == null) {
            throw new NullPointerException();
        }
        return this.properties.getProperty(key);
    }

    public Enumeration<String> getKeys() {
        // TODO
        return null;
    }
}
