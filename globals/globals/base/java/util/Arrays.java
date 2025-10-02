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
        return new ArrayIterator<E>(this.data);
    }
}
