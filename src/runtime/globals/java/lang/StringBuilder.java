package java.lang;

public final class StringBuilder {
    private char[] data;

    public StringBuilder() {
        this.data = new char[0];
    }

    public StringBuilder append(char[] chars) {
        char[] newData = new char[this.data.length + chars.length];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        System.arraycopy(chars, 0, newData, this.data.length, chars.length);

        this.data = newData;

        return this;
    }

    public StringBuilder append(String string) {
        char[] newData = new char[this.data.length + string.length()];
        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        string.getChars(0, string.length(), newData, this.data.length);

        this.data = newData;

        return this;
    }

    public StringBuilder append(int integer) {
        String stringified = Integer.toString(integer);
        return this.append(stringified);
    }

    public String toString() {
        return new String(this.data);
    }
}
