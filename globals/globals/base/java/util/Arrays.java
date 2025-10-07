package java.util;

public class Arrays {
    public static <E> List<E> asList​(E... arr) {
        return new ArrayBackedList<E>(arr);
    }

    public static <T> T[] copyOf​(T[] original, int newLength) {
        Object[] newArray = new Object[newLength];

        int usedLength;
        if (newLength < original.length) {
            usedLength = newLength;
        } else {
            usedLength = original.length;
        }

        System.arraycopy(original, 0, newArray, 0, usedLength);
        return (T[]) newArray;
    }

    public static String toString(Object[] arr) {
        if (arr == null) {
            return "null";
        }

        StringBuilder result = new StringBuilder();
        result.append('[');
        for (int i = 0; i < arr.length; i ++) {
            result.append(String.valueOf(arr[i]));
            if (i != arr.length - 1) {
                result.append(", ");
            }
        }
        result.append(']');
        return result.toString();
    }
}

class ArrayBackedList<E> extends AbstractList<E> {
    private Object[] data;

    ArrayBackedList(Object[] data) {
        this.data = data;
    }

    public E get(int index) {
        return (E) this.data[index];
    }

    public E set(int index, E element) {
        E prev = (E) this.data[index];
        this.data[index] = element;
        return prev;
    }

    public int size() {
        return this.data.length;
    }

    public void clear() {
        // TODO remove this once `AbstractList.clear` is implemented
        throw new UnsupportedOperationException();
    }

    public Iterator<E> iterator() {
        // TODO remove this once `AbstractList.iterator` is implemented
        return new ArrayIterator<E>(this.data, this.data.length);
    }
}
