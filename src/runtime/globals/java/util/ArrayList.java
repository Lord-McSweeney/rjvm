package java.util;

public class ArrayList<E> extends AbstractList<E> implements List<E> {
    public ArrayList() {
        super();
    }

    public ArrayList(Collection<E> collection) {
        super();
    }

    public ArrayList(int capacity) {
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

    public int size() {
        // TODO implement
        return 0;
    }
}
