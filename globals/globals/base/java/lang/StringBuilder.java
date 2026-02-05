package java.lang;

public final class StringBuilder extends AbstractStringBuilder implements CharSequence {
    public StringBuilder() {
        super(16);
    }

    public StringBuilder(int capacity) {
        super(capacity);
    }

    public StringBuilder(String initial) {
        int length = initial.length();

        char[] copyData = new char[length + 8];
        initial.getChars(0, length, copyData, 0);

        this.value = copyData;
        this.count = length;
    }

    public void ensureCapacity(int minimumCapacity) {
        if (this.value.length < minimumCapacity) {
            char[] newData = new char[minimumCapacity + 8];
            System.arraycopy(this.value, 0, newData, 0, this.value.length);
            this.value = newData;
        }
    }

    public StringBuilder append(char character) {
        ensureCapacity(this.count + 1);
        this.value[this.count] = character;

        this.count += 1;

        return this;
    }

    public StringBuilder append(char[] chars) {
        ensureCapacity(this.count + chars.length);

        System.arraycopy(chars, 0, this.value, this.count, chars.length);

        this.count += chars.length;

        return this;
    }

    public StringBuilder append(String string) {
        if (string == null) {
            return this.append("null");
        }

        ensureCapacity(this.count + string.length());

        string.getChars(0, string.length(), this.value, this.count);

        this.count += string.length();

        return this;
    }

    public StringBuilder append(Object object) {
        return this.append(String.valueOf(object));
    }

    public StringBuilder append(int integer) {
        String stringified = Integer.toString(integer);
        return this.append(stringified);
    }

    public StringBuilder append(long l) {
        String stringified = Long.toString(l);
        return this.append(stringified);
    }

    public StringBuilder append(float f) {
        String stringified = Float.toString(f);
        return this.append(stringified);
    }

    public StringBuilder append(double d) {
        String stringified = Double.toString(d);
        return this.append(stringified);
    }

    public StringBuilder append(boolean b) {
        if (b) {
            return this.append("true");
        } else {
            return this.append("false");
        }
    }

    public void setLength(int newLength) {
        if (newLength < 0) {
            throw new IndexOutOfBoundsException();
        }

        char[] newStorage = new char[newLength];

        int smallerLength = Math.min(this.count, newLength);
        System.arraycopy(this.value, 0, newStorage, 0, smallerLength);

        this.value = newStorage;
    }

    public int length() {
        return this.count;
    }

    public int capacity() {
        return this.value.length;
    }

    public String toString() {
        return new String(this.value, 0, this.count);
    }
}
