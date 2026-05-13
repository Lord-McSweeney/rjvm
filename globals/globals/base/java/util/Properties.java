package java.util;

import java.io.InputStream;
import java.io.IOException;

public class Properties extends Hashtable<Object, Object> {
    final int PARSE_KEY = 0;
    final int PARSE_VALUE = 1;

    public Properties() {
        super();
    }

    public void load(InputStream stream) throws IOException {
        // TODO implement comments, escapes, and whitespace handling
        /* Currently implemented states:
            0 = parsing key
            1 = parsing value
        */
        int state = PARSE_KEY;

        char[] buffer = new char[stream.available()];
        int bufferLength = 0;

        String currentKey = null;

        char next = (char) stream.read();
        while (stream.available() > 0) {
            switch (state) {
                case PARSE_KEY:
                    if (next == '\\') {
                        // Skip it

                        // Interpret the next character
                        char literalNext = (char) stream.read();

                        // Newlines are ignored (TODO do we need to check for \r\n?)
                        if (literalNext == '\n') {
                            // Skip any literal whitespace right at the start of
                            // the next line
                            next = (char) stream.read();
                            while (isWhitespace(next)) {
                                next = (char) stream.read();
                            }
                            break;
                        }

                        next = parseEscapedCharacter(literalNext);

                        buffer[bufferLength] = next;
                        bufferLength ++;

                        next = (char) stream.read();
                        break;
                    } else if (next == ':' || next == '=') {
                        currentKey = (new String(buffer, 0, bufferLength)).trim();
                        bufferLength = 0;

                        state = PARSE_VALUE;

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

                case PARSE_VALUE:
                    if (next == '\\') {
                        // Skip it

                        // Interpret the next character
                        char literalNext = (char) stream.read();

                        // Newlines are ignored (TODO do we need to check for \r\n?)
                        if (literalNext == '\n') {
                            // Skip any literal whitespace right at the start of
                            // the next line
                            next = (char) stream.read();
                            while (isWhitespace(next)) {
                                next = (char) stream.read();
                            }
                            break;
                        }

                        next = parseEscapedCharacter(literalNext);

                        buffer[bufferLength] = next;
                        bufferLength ++;

                        next = (char) stream.read();
                        break;
                    } else if (next == '\n') {
                        String value = new String(buffer, 0, bufferLength);
                        bufferLength = 0;

                        super.put(currentKey, value);

                        state = PARSE_KEY;

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

                        state = PARSE_KEY;
                        break;
                    }

                    buffer[bufferLength] = next;
                    bufferLength ++;

                    next = (char) stream.read();
                    break;
            }
        }

        if (state == PARSE_VALUE) {
            // We haven't processed the item in `next` because the loop ended:
            // do it now
            buffer[bufferLength] = next;
            bufferLength ++;

            String value = new String(buffer, 0, bufferLength);

            super.put(currentKey, value);
        }
    }

    private static char parseEscapedCharacter(char c) {
        if (c == 'r') {
            return '\r';
        } else if (c == 'n') {
            return '\n';
        } else if (c == 't') {
            return '\t';
        } else {
            // TODO more
            return c;
        }
    }

    private static boolean isWhitespace(char c) {
        return c == ' ' || c == '\t' || c == '\r' || c == '\n';
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
