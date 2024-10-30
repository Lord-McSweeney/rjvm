package java.lang;

public final class StringBuilder {
    private char[] data;

    public StringBuilder() {
        this.data = new char[0];
    }

    public StringBuilder append(String string) {
        char[] newData = new char[this.data.length + string.length()];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        string.getChars(0, string.length(), newData, this.data.length);

        this.data = newData;

        return this;
    }

    public StringBuilder append(int integer) {
        // TODO
        return this.append("[integer]");
    }

    public String toString() {
        return new String(this.data);
    }
}
