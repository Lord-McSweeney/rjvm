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
        // TODO implement
        return CharBuffer.wrap(new char[0]);
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
