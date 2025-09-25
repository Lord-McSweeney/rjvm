package java.nio;

public final class ByteOrder {
    public static final ByteOrder BIG_ENDIAN = new ByteOrder();
    public static final ByteOrder LITTLE_ENDIAN = new ByteOrder();

    private ByteOrder() { }

    public static ByteOrder nativeOrder() {
        // FIXME do we even support BE?
        return ByteOrder.LITTLE_ENDIAN;
    }

    public String toString() {
        if (this == ByteOrder.BIG_ENDIAN) {
            return "BIG_ENDIAN";
        } else {
            return "LITTLE_ENDIAN";
        }
    }
}
