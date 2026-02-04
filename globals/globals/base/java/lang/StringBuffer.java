package java.lang;

public final class StringBuffer extends AbstractStringBuilder implements CharSequence {
    public StringBuffer() {
        super(16);
    }

    public StringBuffer(int capacity) {
        super(capacity);
    }

    public StringBuffer(String initial) {
        int length = initial.length();

        char[] copyData = new char[length + 8];
        initial.getChars(0, length, copyData, 0);

        this.value = copyData;
        this.count = length;
    }

    public synchronized void ensureCapacity(int minimumCapacity) {
        if (this.value.length < minimumCapacity) {
            char[] newData = new char[minimumCapacity + 8];
            System.arraycopy(this.value, 0, newData, 0, this.value.length);
            this.value = newData;
        }
    }

    public synchronized StringBuffer append(char character) {
        ensureCapacity(this.count + 1);
        this.value[this.count] = character;

        this.count += 1;

        return this;
    }

    public synchronized StringBuffer append(char[] chars) {
        ensureCapacity(this.count + chars.length);

        System.arraycopy(chars, 0, this.value, this.count, chars.length);

        this.count += chars.length;

        return this;
    }

    public synchronized StringBuffer append(String string) {
        if (string == null) {
            return this.append("null");
        }

        ensureCapacity(this.count + string.length());

        string.getChars(0, string.length(), this.value, this.count);

        this.count += string.length();

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

    public StringBuffer append(double d) {
        String stringified = Double.toString(d);
        return this.append(stringified);
    }

    public StringBuffer append(boolean b) {
        if (b) {
            return this.append("true");
        } else {
            return this.append("false");
        }
    }

    public synchronized void setLength(int newLength) {
        if (newLength < 0) {
            throw new IndexOutOfBoundsException();
        }

        char[] newStorage = new char[newLength];

        int smallerLength = Math.min(this.count, newLength);
        System.arraycopy(this.value, 0, newStorage, 0, smallerLength);

        this.value = newStorage;
    }

    public synchronized int length() {
        return this.count;
    }

    public synchronized int capacity() {
        return this.value.length;
    }

    public synchronized String toString() {
        return new String(this.value, 0, this.count);
    }
}
