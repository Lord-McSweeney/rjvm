package java.lang;

abstract class AbstractStringBuilder implements Appendable, CharSequence {
    char[] value;

    int count;

    AbstractStringBuilder() { }

    AbstractStringBuilder(int capacity) {
        this.value = new char[capacity];
    }

    public int length() {
        return this.count;
    }

    public char charAt(int index) {
        if (index >= count) {
            throw new IndexOutOfBoundsException();
        }

        return this.value[index];
    }

    public CharSequence subSequence(int start, int end) {
        return this.substring(start, end);
    }

    public String substring(int start, int end) {
        if (start < 0 || end > count || start > end) {
            throw new IndexOutOfBoundsException();
        }
        return new String(this.value, start, end - start);
    }

    public abstract String toString();
}
