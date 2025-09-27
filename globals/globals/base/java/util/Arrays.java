package java.util;

public class Arrays {
    public static <E> List<E> asList​(E... arr) {
        return new ArrayBackedList<E>(arr);
    }
}

class ArrayBackedList<E> implements List<E> {
    private Object[] data;

    ArrayBackedList(Object[] data) {
        this.data = data;
    }

    public boolean add(E e) {
        throw new UnsupportedOperationException();
    }

    public void add(int index, E element) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(Collection<? extends E> collection) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(int index, Collection<? extends E> collection) {
        throw new UnsupportedOperationException();
    }

    public E get(int index) {
        return (E) this.data[index];
    }

    public E set(int index, E element) {
        E prev = (E) this.data[index];
        this.data[index] = element;
        return prev;
    }

    public E remove(int index) {
        throw new UnsupportedOperationException();
    }

    public int size() {
        return this.data.length;
    }

    public void clear() {
        throw new UnsupportedOperationException();
    }

    public boolean isEmpty() {
        return this.data.length == 0;
    }

    public Iterator<E> iterator() {
        return new ArrayIterator<E>(this.data);
    }
}
