package java.nio.charset;

import java.nio.ByteBuffer;
import java.nio.CharBuffer;

public abstract class Charset {
    public static Charset forName(String name) {
        // TODO implement properly
        return new Utf8Charset();
    }

    public static Charset defaultCharset() {
        // TODO implement properly
        return new Utf8Charset();
    }

    public final CharBuffer decode(ByteBuffer buf) {
        // TODO implement properly
        byte[] data = buf.array();
        char[] newData = new char[data.length];
        for (int i = 0; i < data.length; i ++) {
            newData[i] = (char) data[i];
        }
        return CharBuffer.wrap(newData);
    }

    public final ByteBuffer encode(CharBuffer buf) {
        // TODO implement
        return null;
    }

    // TODO implement proper decoding; this is not a real part of the API
    public static native byte[] stringToUtf8(String string);
}

class Utf8Charset extends Charset {
    Utf8Charset() { }
}
