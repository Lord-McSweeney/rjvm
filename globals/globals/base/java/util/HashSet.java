package java.util;

// FIXME implement capacity
public class HashSet<E> extends AbstractSet<E> implements Set<E> {
    HashMapKeySet<E> backingSet;

    public HashSet() {
        this.backingSet = new HashMapKeySet<E>(new HashMap<E, Object>(), true);
    }

    public HashSet(int capacity) {
        this.backingSet = new HashMapKeySet<E>(new HashMap<E, Object>(capacity), true);
    }

    public HashSet(Collection<? extends E> collection) {
        this(collection.size());
        this.addAll(collection);
    }

    public boolean add(E element) {
        return this.backingSet.add(element);
    }

    public void clear() {
        this.backingSet.clear();
    }

    public boolean contains(Object element) {
        return this.backingSet.contains(element);
    }

    public Iterator<E> iterator() {
        return this.backingSet.iterator();
    }

    public int size() {
        return this.backingSet.size();
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
