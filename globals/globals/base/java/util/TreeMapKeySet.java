package java.util;

class TreeMapKeySet<E> extends AbstractSet<E> implements Cloneable {
    private TreeMap<E, Object> backingMap;
    private boolean writable;

    private static Object markerObject = new Object();

    // TODO: `writeable` should actually be a "addable" flag, since entries can
    // still be removed from a non-writeable TreeMapKeySet
    TreeMapKeySet(TreeMap backingMap, boolean writable) {
        this.backingMap = backingMap;
        this.writable = writable;
    }

    public boolean add(E element) {
        if (this.writable) {
            Object result = this.backingMap.put(element, TreeMapKeySet.markerObject);
            return result == null;
        } else {
            throw new UnsupportedOperationException();
        }
    }

    public boolean contains(Object element) {
        return this.backingMap.containsKey(element);
    }

    public int size() {
        return this.backingMap.size();
    }

    public Iterator<E> iterator() {
        return new TreeMapKeyIterator<E>(this.backingMap);
    }

    class TreeMapKeyIterator<E> implements Iterator<E> {
        private TreeMap.Entry nextEntry;

        public TreeMapKeyIterator(TreeMap<E, ?> map) {
            this.nextEntry = (TreeMap.Entry) map.firstEntry();
        }

        public boolean hasNext() {
            return this.nextEntry != null;
        }

        public E next() {
            TreeMap.Entry result = this.nextEntry;
            if (result == null) {
                throw new NoSuchElementException();
            }

            this.nextEntry = TreeMap.findNextEntry(result);

            return (E) result.key;
        }
    }
}
