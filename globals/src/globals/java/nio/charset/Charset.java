package java.nio.charset;

public abstract class Charset {
    public static Charset forName(String name) {
        // TODO implement
        return null;
    }

    public static Charset defaultCharset() {
        // TODO implement
        return null;
    }

    // TODO implement proper decoding; this is not a real part of the API
    public static native byte[] stringToUtf8(String string);
}
