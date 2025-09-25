package java.util;

public abstract class AbstractList<E> extends AbstractCollection<E> implements List<E> {
    protected AbstractList() { }

    public boolean add(E element) {
        this.add(this.size(), element);
        return true;
    }

    public void add(int index, E element) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(int index, Collection<? extends E> collection) {
        // TODO implement
        return false;
    }

    public abstract E get(int index);

    public E set(int index, E element) {
        throw new UnsupportedOperationException();
    }

    public E remove(int index) {
        throw new UnsupportedOperationException();
    }

    public void clear() {
        // TODO implement
    }

    public Iterator<E> iterator() {
        // TODO implement
        return null;
    }
}
