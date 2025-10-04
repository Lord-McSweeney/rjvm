package java.util;

import rjvm.internal.Todo;

public class TreeMap<K, V> extends AbstractMap<K, V> {
    Entry<K, V> root;
    int size;

    // The comparator we're using, or `null` if we use default comparison
    Comparator<? super K> comparator;

    public TreeMap(Comparator<? super K> comparator) {
        Todo.warnNotImpl("java.util.TreeMap(comparator)");

        this.comparator = comparator;
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return new EntrySet(this);
    }

    public K firstKey() {
        if (this.root == null) {
            return null;
        }

        Entry<K, V> current = this.root;
        while (current.left != null) {
            current = current.left;
        }

        return current.key;
    }

    public K lastKey() {
        if (this.root == null) {
            return null;
        }

        Entry<K, V> current = this.root;
        while (current.right != null) {
            current = current.right;
        }

        return current.key;
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

    public boolean containsKey(Object key) {
        Comparable<K> comparableKey = (Comparable<K>) key;

        if (this.root == null) {
            return false;
        } else {
            Entry<K, V> current = this.root;
            while (true) {
                int result = comparableKey.compareTo(current.key);
                if (result == 0) {
                    return true;
                } else if (result > 0) {
                    if (current.right == null) {
                        return false;
                    } else {
                        current = current.right;
                    }
                } else {
                    if (current.left == null) {
                        return false;
                    } else {
                        current = current.left;
                    }
                }
            }
        }
    }

    public V get(Object key) {
        Comparable<K> comparableKey = (Comparable<K>) key;

        if (this.root == null) {
            return null;
        } else {
            Entry<K, V> current = this.root;
            while (true) {
                int result = comparableKey.compareTo(current.key);
                if (result == 0) {
                    return current.value;
                } else if (result > 0) {
                    if (current.right == null) {
                        return null;
                    } else {
                        current = current.right;
                    }
                } else {
                    if (current.left == null) {
                        return null;
                    } else {
                        current = current.left;
                    }
                }
            }
        }
    }

    public V put(K key, V value) {
        Comparable<K> comparableKey = (Comparable<K>) key;

        if (this.root == null) {
            Entry<K, V> entry = new Entry<K, V>(key, value);
            this.root = entry;

            return null;
        } else {
            Entry<K, V> current = this.root;
            while (true) {
                int result = comparableKey.compareTo(current.key);
                if (result == 0) {
                    V previousValue = current.value;
                    current.value = value;
                    return previousValue;
                } else if (result > 0) {
                    if (current.right == null) {
                        Entry<K, V> entry = new Entry<K, V>(key, value);
                        current.right = entry;

                        return null;
                    } else {
                        current = current.right;
                    }
                } else {
                    if (current.left == null) {
                        Entry<K, V> entry = new Entry<K, V>(key, value);
                        current.left = entry;

                        return null;
                    } else {
                        current = current.left;
                    }
                }
            }
        }
    }

    public void clear() {
        this.root = null;
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
