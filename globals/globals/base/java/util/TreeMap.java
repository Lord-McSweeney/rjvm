package java.util;

import rjvm.internal.Todo;

public class TreeMap<K, V> extends AbstractMap<K, V> implements Cloneable {
    Entry<K, V> root;
    int size;

    // The comparator we're using
    Comparator comparator;

    private static Comparator naturalComparator;
    static {
        TreeMap.naturalComparator = new Comparator() {
            public int compare(Object o1, Object o2) {
                Comparable comparableO1 = (Comparable) o1;
                return comparableO1.compareTo(o2);
            }

            public boolean equals(Object other) {
                // TODO ???
                return this == other;
            }
        };
    }

    public TreeMap() {
        this(TreeMap.naturalComparator);
    }

    public TreeMap(Comparator<? super K> comparator) {
        this.comparator = comparator;
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return new EntrySet(this);
    }

    public Set<K> keySet() {
        // Return a non-writeable TreeMapKeySet
        return new TreeMapKeySet<K>(this, false);
    }

    public Map.Entry<K, V> firstEntry() {
        if (this.root == null) {
            return null;
        }

        Entry<K, V> current = this.root;
        while (current.left != null) {
            current = current.left;
        }

        return current;
    }

    public K firstKey() {
        Map.Entry<K, V> entry = this.firstEntry();
        if (entry != null) {
            return entry.getKey();
        } else {
            return null;
        }
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
                int result = this.comparator.compare(comparableKey, current.key);
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
                int result = this.comparator.compare(comparableKey, current.key);
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
            this.size += 1;

            Entry<K, V> entry = new Entry<K, V>(key, value);
            this.root = entry;

            return null;
        } else {
            Entry<K, V> current = this.root;
            while (true) {
                int result = this.comparator.compare(comparableKey, current.key);
                if (result == 0) {
                    V previousValue = current.value;
                    current.value = value;
                    return previousValue;
                } else if (result > 0) {
                    if (current.right == null) {
                        this.size += 1;

                        Entry<K, V> entry = new Entry<K, V>(key, value);
                        current.right = entry;
                        entry.parent = current;

                        return null;
                    } else {
                        current = current.right;
                    }
                } else {
                    if (current.left == null) {
                        this.size += 1;

                        Entry<K, V> entry = new Entry<K, V>(key, value);
                        current.left = entry;
                        entry.parent = current;

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

    // Given an entry, return the next entry. For use by `TreeSet`
    static TreeMap.Entry findNextEntry(TreeMap.Entry current) {
        if (current.right != null) {
            TreeMap.Entry currentLeft = current.right;
            while (currentLeft.left != null) {
                currentLeft = currentLeft.left;
            }
            return currentLeft;
        } else {
            TreeMap.Entry currentParent = current.parent;
            TreeMap.Entry currentNode = current;
            while (currentParent != null && currentParent.right == currentNode) {
                currentNode = currentParent;
                currentParent = currentParent.parent;
            }
            return currentParent;
        }
    }

    class EntrySet extends AbstractSet<Map.Entry<K, V>> {
        // TODO implement this class

        TreeMap map;

        EntrySet(TreeMap map) {
            this.map = map;
        }

        public Iterator<Map.Entry<K, V>> iterator() {
            Todo.warnNotImpl("java.util.TreeMap.EntrySet.iterator");

            return new ArrayIterator(new Object[0]);
        }

        public int size() {
            return map.size;
        }
    }

    static class Entry<K, V> implements Map.Entry<K, V> {
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
