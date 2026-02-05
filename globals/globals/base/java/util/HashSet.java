package java.util;

// FIXME implement capacity
public class HashSet<E> extends AbstractSet<E> implements Set<E> {
    private static Object markerObject = new Object();

    HashMap<E, Object> backingMap;

    public HashSet() {
        this.backingMap = new HashMap<E, Object>();
    }

    public HashSet(int capacity) {
        this.backingMap = new HashMap<E, Object>(capacity);
    }

    public HashSet(Collection<? extends E> collection) {
        this(collection.size());
        this.addAll(collection);
    }

    public boolean add(E element) {
        return this.backingMap.put(element, markerObject) == null;
    }

    public void clear() {
        this.backingMap.clear();
    }

    public boolean contains(Object element) {
        return this.backingMap.containsKey((E) element);
    }

    public Iterator<E> iterator() {
        return this.backingMap.keySet().iterator();
    }

    public int size() {
        return this.backingMap.size();
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
