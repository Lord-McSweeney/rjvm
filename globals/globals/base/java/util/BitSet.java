package java.util;

public class BitSet implements Cloneable {
    // This is inefficient (each byte stores a single bit), but it's good enough for now
    private boolean[] bits;

    public BitSet() {
        this.bits = new boolean[0];
    }

    public BitSet(int numBits) {
        this.bits = new boolean[numBits];
    }

    public void set(int index) {
        // TODO auto-expand
        this.bits[index] = true;
    }

    public boolean get(int index) {
        // This does not auto-expand
        if (index < this.bits.length) {
            return this.bits[index];
        } else {
            return false;
        }
    }
}
