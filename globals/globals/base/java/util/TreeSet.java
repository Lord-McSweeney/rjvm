package java.util;

import rjvm.internal.Todo;

public class TreeSet<E> extends AbstractSet<E> implements Cloneable {
    private TreeMapKeySet<E> backingSet;

    public TreeSet() {
        this.backingSet = new TreeMapKeySet<E>(new TreeMap<E, Object>(), true);
    }

    public boolean add(E element) {
        return this.backingSet.add(element);
    }

    public boolean contains(Object element) {
        return this.backingSet.contains(element);
    }

    public int size() {
        return this.backingSet.size();
    }

    public Iterator<E> iterator() {
        return this.backingSet.iterator();
    }
}
