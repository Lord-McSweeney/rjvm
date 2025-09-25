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

    public abstract Iterator<E> iterator();

    public abstract int size();

    public boolean isEmpty() {
        return size() == 0;
    }

    public String toString() {
        StringBuilder builder = new StringBuilder();
        builder.append('[');

        Iterator<E> iter = this.iterator();
        while (iter.hasNext()) {
            E next = iter.next();
            builder.append(next.toString());

            // Append comma only if there's a next element
            if (iter.hasNext()) {
                builder.append(", ");
            }
        }

        builder.append(']');

        return builder.toString();
    }
}
