package java.util;

public abstract class AbstractCollection<E> implements Collection<E> {
    protected AbstractCollection() { }

    public boolean add(E element) {
        throw new UnsupportedOperationException();
    }

    public void clear() {
        // TODO implement
    }

    public abstract int size();

    public boolean isEmpty() {
        return size() == 0;
    }
}
