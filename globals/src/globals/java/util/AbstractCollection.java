package java.util;

public abstract class AbstractCollection<E> implements Collection<E> {
    protected AbstractCollection() { }

    public boolean add(E element) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(Collection<? extends E> collection) {
        boolean added = false;

        Iterator<? extends E> iterator = collection.iterator();
        while (iterator.hasNext()) {
            E next = (E) iterator.next();
            this.add(next);
            added = true;
        }

        return added;
    }

    public void clear() {
        // TODO implement
    }

    public abstract int size();

    public boolean isEmpty() {
        return size() == 0;
    }
}
