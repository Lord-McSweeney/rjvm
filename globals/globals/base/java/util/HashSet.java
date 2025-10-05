package java.util;

// FIXME implement capacity
public class HashSet<E> extends AbstractSet<E> implements Set<E> {
    Object[] data;

    public HashSet() {
        this.data = new Object[0];
    }

    public HashSet(int capacity) {
        // TODO implement capacity
        this.data = new Object[0];
    }

    public HashSet(Collection<? extends E> collection) {
        // FIXME use capacity here
        this.data = new Object[0];
        this.addAll(collection);
    }

    public boolean add(E element) {
        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i] == null) {
                if (element == null) {
                    return false;
                } else {
                    continue;
                }
            }

            if (this.data[i].equals(element)) {
                return false;
            }
        }

        int oldLength = this.data.length;

        Object[] newData = new Object[oldLength + 1];

        System.arraycopy(this.data, 0, newData, 0, oldLength);
        newData[oldLength] = element;

        this.data = newData;

        return true;
    }

    public void clear() {
        this.data = new Object[0];
    }

    public boolean contains(Object element) {
        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i] == null) {
                if (element == null) {
                    return true;
                }
            } else if (this.data[i].equals(element)) {
                return true;
            }
        }

        return false;
    }

    public Iterator<E> iterator() {
        return new ArrayIterator(this.data);
    }

    public int size() {
        return this.data.length;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
