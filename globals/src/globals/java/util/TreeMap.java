package java.util;

public class TreeMap<K, V> extends AbstractMap<K, V> {
    Object[] keys;
    Object[] values;

    EntrySet cachedEntrySet;

    public TreeMap(Comparator<? super K> comparator) {
        this.keys = new Object[0];
        this.values = new Object[0];
    }

    public Set<Entry<K, V>> entrySet() {
        if (this.cachedEntrySet == null) {
            this.cachedEntrySet = new EntrySet();
        }
        return this.cachedEntrySet;
    }

    public K firstKey() {
        // TODO implement
        return null;
    }

    public K lastKey() {
        // TODO implement
        return null;
    }

    public SortedMap<K, V> headMap(K highestKey) {
        // TODO implement
        return null;
    }

    public SortedMap<K, V> tailMap(K lowestKey) {
        // TODO implement
        return null;
    }

    public SortedMap<K, V> subMap(K lowestKey, K highestKey) {
        // TODO implement
        return null;
    }

    class EntrySet extends AbstractSet<Entry<K, V>> {
        // TODO implement

        public Iterator<Entry<K, V>> iterator() {
            // TODO implement
            return new ArrayIterator(new Object[0]);
        }

        public int size() {
            // TODO implement
            return 0;
        }
    }
}
