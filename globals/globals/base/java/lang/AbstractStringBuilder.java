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

    public abstract String toString();

    public String substring(int start, int end) {
        if (start < 0 || end > count || start > end) {
            throw new IndexOutOfBoundsException();
        }
        return new String(this.value, start, end - start);
    }

    public int indexOf(String string) {
        return this.indexOf(string, 0);
    }

    public int indexOf(String search, int fromIndex) {
        if (search.length() > this.length()) {
            return -1;
        }

        if (fromIndex < 0) {
            fromIndex = 0;
        }

        for (int i = fromIndex; i <= count - search.length(); i ++) {
            boolean failedToMatch = false;
            for (int j = 0; j < search.length(); j ++) {
                if (this.value[i + j] != search.charAt(j)) {
                    failedToMatch = true;
                    break;
                }
            }

            if (!failedToMatch) {
                return i;
            }
        }

        return -1;
    }
}
