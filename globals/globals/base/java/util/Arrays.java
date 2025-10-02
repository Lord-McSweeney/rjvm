package java.util;

public class Arrays {
    public static <E> List<E> asList​(E... arr) {
        return new ArrayBackedList<E>(arr);
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
