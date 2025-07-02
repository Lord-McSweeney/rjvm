package java.util;

import rjvm.internal.Todo;

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
        Todo.warnNotImpl("java.util.TreeMap.firstKey");

        return null;
    }

    public K lastKey() {
        Todo.warnNotImpl("java.util.TreeMap.lastKey");

        return null;
    }

    public SortedMap<K, V> headMap(K highestKey) {
        Todo.warnNotImpl("java.util.TreeMap.headMap");

        return null;
    }

    public SortedMap<K, V> tailMap(K lowestKey) {
        Todo.warnNotImpl("java.util.TreeMap.tailMap");

        return null;
    }

    public SortedMap<K, V> subMap(K lowestKey, K highestKey) {
        Todo.warnNotImpl("java.util.TreeMap.subMap");

        return null;
    }

    class EntrySet extends AbstractSet<Entry<K, V>> {
        // TODO implement

        public Iterator<Entry<K, V>> iterator() {
            Todo.warnNotImpl("java.util.TreeMap.EntrySet.iterator");

            return new ArrayIterator(new Object[0]);
        }

        public int size() {
            Todo.warnNotImpl("java.util.TreeMap.EntrySet.size");

            return 0;
        }
    }
}
