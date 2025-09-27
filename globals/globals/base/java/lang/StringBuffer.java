package java.lang;

// TODO capacity
public final class StringBuffer {
    private char[] data;

    public StringBuffer() {
        this.data = new char[0];
    }

    public StringBuffer(int capacity) {
        this.data = new char[0];
    }

    public StringBuffer(String initial) {
        int length = initial.length();

        char[] copyData = new char[length];
        initial.getChars(0, length, copyData, 0);

        this.data = copyData;
    }

    public StringBuffer append(char character) {
        char[] newData = new char[this.data.length + 1];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);
        newData[this.data.length] = character;

        this.data = newData;

        return this;
    }

    public StringBuffer append(char[] chars) {
        char[] newData = new char[this.data.length + chars.length];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        System.arraycopy(chars, 0, newData, this.data.length, chars.length);

        this.data = newData;

        return this;
    }

    public StringBuffer append(String string) {
        if (string == null) {
            return this.append("null");
        }

        char[] newData = new char[this.data.length + string.length()];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        string.getChars(0, string.length(), newData, this.data.length);

        this.data = newData;

        return this;
    }

    public StringBuffer append(Object object) {
        return this.append(String.valueOf(object));
    }

    public StringBuffer append(int integer) {
        String stringified = Integer.toString(integer);
        return this.append(stringified);
    }

    public StringBuffer append(long l) {
        String stringified = Long.toString(l);
        return this.append(stringified);
    }

    public int length() {
        return this.data.length;
    }

    public String toString() {
        return new String(this.data);
    }
}
