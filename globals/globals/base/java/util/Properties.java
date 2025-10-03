package java.util;

import java.io.InputStream;
import java.io.IOException;

public class Properties extends Hashtable<Object, Object> {
    public Properties() {
        super();
    }

    public void load(InputStream stream) throws IOException {
        // TODO implement comments, escapes, and whitespace handling
        /* Currently implemented states:
            0 = parsing key
            1 = parsing value
        */
        int state = 0;

        char[] buffer = new char[stream.available()];
        int bufferLength = 0;

        String currentKey = null;

        char next = (char) stream.read();
        while (stream.available() > 0) {
            switch (state) {
                case 0:
                    if (next == ':' || next == '=') {
                        currentKey = (new String(buffer, 0, bufferLength)).trim();
                        bufferLength = 0;

                        state = 1;

                        next = (char) stream.read();
                        break;
                    } else if (next == '\n') {
                        bufferLength = 0;

                        next = (char) stream.read();
                        break;
                    } else if (next == '\r') {
                        // Skip over '\n' if it's there
                        next = (char) stream.read();
                        if (next == '\n') {
                            next = (char) stream.read();
                        }

                        bufferLength = 0;

                        break;
                    }

                    buffer[bufferLength] = next;
                    bufferLength ++;

                    next = (char) stream.read();
                    break;

                case 1:
                    if (next == '\n') {
                        String value = new String(buffer, 0, bufferLength);
                        bufferLength = 0;

                        super.put(currentKey, value);

                        state = 0;

                        next = (char) stream.read();
                        break;
                    } else if (next == '\r') {
                        // Skip over '\n' if it's there
                        next = (char) stream.read();
                        if (next == '\n') {
                            next = (char) stream.read();
                        }

                        String value = new String(buffer, 0, bufferLength);
                        bufferLength = 0;

                        super.put(currentKey, value);

                        state = 0;
                        break;
                    }

                    buffer[bufferLength] = next;
                    bufferLength ++;

                    next = (char) stream.read();
                    break;
            }
        }

        if (state == 1) {
            // We haven't processed the item in `next` because the loop ended:
            // do it now
            buffer[bufferLength] = next;
            bufferLength ++;

            String value = new String(buffer, 0, bufferLength);

            super.put(currentKey, value);
        }
    }

    public String getProperty(String key) {
        return this.getProperty(key, null);
    }

    public String getProperty(String key, String defaultValue) {
        Object result = super.get(key);
        if (result == null) {
            // `Hashtable` only returns `null` when the property isn't found
            return defaultValue;
        } else {
            return (String) result;
        }
    }

    public Object setProperty(String key, String value) {
        return super.put(key, value);
    }
}
