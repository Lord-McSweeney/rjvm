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

    public void clear() {
        // TODO implement
    }

    public abstract E get(int index);
}
