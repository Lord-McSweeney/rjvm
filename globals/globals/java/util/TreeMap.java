package java.util;

import rjvm.internal.Todo;

public class TreeMap<K, V> extends AbstractMap<K, V> {
    Entry root;
    int size;

    public TreeMap(Comparator<? super K> comparator) {
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return new EntrySet(this);
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

    public V put(K key, V value) {
        Todo.warnNotImpl("java.util.TreeMap.put");

        return null;
    }

    public void clear() {
        Todo.warnNotImpl("java.util.TreeMap.clear");
    }

    class EntrySet extends AbstractSet<Map.Entry<K, V>> {
        // TODO implement

        TreeMap map;

        EntrySet(TreeMap map) {
            this.map = map;
        }

        public Iterator<Map.Entry<K, V>> iterator() {
            Todo.warnNotImpl("java.util.TreeMap.EntrySet.iterator");

            return new ArrayIterator(new Object[0]);
        }

        public int size() {
            Todo.warnNotImpl("java.util.TreeMap.EntrySet.size");

            return 0;
        }
    }

    static class Entry<K, V> implements Map.Entry<K, V> {
        // TODO implement

        K key;
        V value;

        Entry<K, V> left;
        Entry<K, V> right;

        Entry<K,V> parent;

        Entry(K key, V value) {
            this.key = key;
            this.value = value;
        }

        public K getKey() {
            return this.key;
        }

        public V getValue() {
            return this.value;
        }
    }
}
