package java.util;

public class Vector<E> extends AbstractList<E> {
    public Vector() {
        super();
    }

    public boolean add(E element) {
        this.add(this.size(), element);
        return true;
    }

    public void add(int index, E element) {
        // TODO implement
    }

    public E get(int index) {
        // TODO implement
        return null;
    }

    public E set(int index, E element) {
        // TODO implement
        return null;
    }

    public E remove(int index) {
        // TODO implement
        return null;
    }

    public void clear() {
        // TODO implement
    }

    public int size() {
        // TODO implement
        return 0;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
