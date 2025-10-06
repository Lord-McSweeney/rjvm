package java.util;

class HashMapKeySet<E> extends AbstractSet<E> implements Cloneable {
    private HashMap<E, Object> backingMap;
    private boolean addable;

    private static Object markerObject = new Object();

    HashMapKeySet(HashMap backingMap, boolean addable) {
        this.backingMap = backingMap;
        this.addable = addable;
    }

    public boolean add(E element) {
        if (this.addable) {
            Object result = this.backingMap.put(element, HashMapKeySet.markerObject);
            return result == null;
        } else {
            throw new UnsupportedOperationException();
        }
    }

    public boolean contains(Object element) {
        return this.backingMap.containsKey((E) element);
    }

    public void clear() {
        this.backingMap.clear();
    }

    public int size() {
        return this.backingMap.size();
    }

    public Iterator<E> iterator() {
        return new HashMapKeyIterator<E>(this.backingMap);
    }

    class HashMapKeyIterator<E> implements Iterator<E> {
        private HashMap<E, ?> backingMap;
        private int next;

        public HashMapKeyIterator(HashMap<E, ?> backingMap) {
            this.backingMap = backingMap;
            this.next = 0;
        }

        public boolean hasNext() {
            return this.next != this.backingMap.size();
        }

        public E next() {
            if (this.next == this.backingMap.size()) {
                throw new NoSuchElementException();
            }

            return this.backingMap.keyByIndex(this.next ++);
        }
    }
}
