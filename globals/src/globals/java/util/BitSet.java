package java.util;

public class BitSet {
    // This is inefficient (each byte stores a single bit), but it's good enough for now
    private byte[] bits;

    public BitSet() {
        this.bits = new byte[0];
    }

    public BitSet(int numBits) {
        this.bits = new byte[numBits];
    }
}
