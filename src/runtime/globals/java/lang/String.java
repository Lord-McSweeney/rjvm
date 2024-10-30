package java.lang;

public final class String {
    private char[] data;

    public String(char[] data) {
        int length = data.length;
        char[] copyData = new char[length];
        System.arraycopy(data, 0, copyData, 0, length);

        this.data = copyData;
    }

    public void getChars(int srcBegin, int srcEnd, char[] dst, int dstBegin) {
       if (
            srcBegin < 0 ||
            srcBegin > srcEnd ||
            srcEnd > this.data.length
            // Other conditions will be checked for by System.arraycopy
        ) {
            throw new IndexOutOfBoundsException();
        }

        System.arraycopy(this.data, srcBegin, dst, dstBegin, srcEnd - srcBegin);
    }

    public int length() {
        return this.data.length;
    }
}
