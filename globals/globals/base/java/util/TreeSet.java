package java.util;

import rjvm.internal.Todo;

public class TreeSet<E> extends AbstractSet<E> implements Cloneable {
    private TreeMap<E, Object> backingMap;

    private static Object markerObject = new Object();

    public TreeSet() {
        this.backingMap = new TreeMap<E, Object>();
    }

    public boolean add(E element) {
        Object result = this.backingMap.put(element, TreeSet.markerObject);
        return result == null;
    }

    public boolean contains(Object element) {
        return this.backingMap.containsKey(element);
    }

    public int size() {
        return this.backingMap.size();
    }

    public Iterator<E> iterator() {
        return new TreeSetIterator<E>(this.backingMap);
    }

    class TreeSetIterator<E> implements Iterator<E> {
        private TreeMap.Entry nextEntry;

        public TreeSetIterator(TreeMap<E, ?> map) {
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
