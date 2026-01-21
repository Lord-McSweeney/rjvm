package java.lang;

abstract class AbstractStringBuilder implements Appendable, CharSequence {
    char[] value;

    int count;

    AbstractStringBuilder() { }

    AbstractStringBuilder(int capacity) {
        this.value = new char[capacity];
    }

    public int length() {
        return count;
    }

    public abstract String toString();
}
