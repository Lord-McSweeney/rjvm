package java.util;

import rjvm.internal.Todo;

public class HashMap<K, V> extends AbstractMap<K, V> {
    // FIXME we should really not be reimplementing all the AbstractMap functions...
    // TODO actually implement a hashmap

    Object[] keys;
    Object[] values;

    public HashMap() {
        this.keys = new Object[0];
        this.values = new Object[0];
    }

    public HashMap(int capacity) {
        // TODO implement capacity
        this.keys = new Object[0];
        this.values = new Object[0];
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return new HashMapEntrySet<K, V>(this);
    }

    public Set<K> keySet() {
        return new HashMapKeySet<K>(this, false);
    }

    public Collection<V> values() {
        return Arrays.asList((V[]) this.values);
    }

    public boolean containsKey(K key) {
        for (int i = 0; i < this.keys.length; i ++) {
            if (this.keys[i] == null) {
                if (key == null) {
                    return true;
                } else {
                    continue;
                }
            }

            if (this.keys[i].equals(key)) {
                return true;
            }
        }

        return false;
    }

    public V get(Object key) {
        for (int i = 0; i < this.keys.length; i ++) {
            if (this.keys[i] == null) {
                if (key == null) {
                    return (V) this.values[i];
                } else {
                    continue;
                }
            }

            if (this.keys[i].equals(key)) {
                return (V) this.values[i];
            }
        }

        return null;
    }

    public V put(K key, V value) {
        for (int i = 0; i < this.keys.length; i ++) {
            if (this.keys[i] == null) {
                if (key == null) {
                    // The key was present, so replace the old value and return it
                    Object oldValue = this.values[i];
                    this.values[i] = value;
                    return (V) oldValue;
                } else {
                    continue;
                }
            }

            if (this.keys[i].equals(key)) {
                // The key was present, so replace the old value and return it
                Object oldValue = this.values[i];
                this.values[i] = value;
                return (V) oldValue;
            }
        }

        // The key wasn't present, so resize the arrays, insert the
        // key and value at the end, and return null.
        int newSize = this.size() + 1;
        Object[] newKeys = new Object[newSize];
        Object[] newValues = new Object[newSize];

        System.arraycopy(this.keys, 0, newKeys, 0, this.keys.length);
        System.arraycopy(this.values, 0, newValues, 0, this.values.length);

        newKeys[newSize - 1] = key;
        newValues[newSize - 1] = value;

        this.keys = newKeys;
        this.values = newValues;

        return null;
    }

    public V remove(Object key) {
        int pos = -1;
        Object previousValue = null;

        for (int i = 0; i < this.keys.length; i ++) {
            if (this.keys[i] == null) {
                if (key == null) {
                    // The key was present, so record the index
                    pos = i;
                    previousValue = this.values[i];
                    break;
                } else {
                    continue;
                }
            }

            if (this.keys[i].equals(key)) {
                pos = i;
                previousValue = this.values[i];
                break;
            }
        }

        if (pos == -1) {
            return null;
        }
        // The key wasn't present, so resize the arrays, insert the
        // key and value at the end, and return null.
        int newSize = this.size() - 1;
        Object[] newKeys = new Object[newSize];
        Object[] newValues = new Object[newSize];

        System.arraycopy(this.keys, 0, newKeys, 0, pos);
        if (pos != newSize) {
            System.arraycopy(this.keys, pos + 1, newKeys, pos + 1, newSize - pos);
        }

        System.arraycopy(this.values, 0, newValues, 0, pos);
        if (pos != newSize) {
            System.arraycopy(this.values, pos + 1, newValues, pos + 1, newSize - pos);
        }

        this.keys = newKeys;
        this.values = newValues;

        return (V) previousValue;
    }

    K keyByIndex(int index) {
        return (K) this.keys[index];
    }

    public void putAll(Map<? extends K, ? extends V> map) {
        Todo.warnNotImpl("java.util.HashMap.putAll");
    }

    public int size() {
        return this.keys.length;
    }

    public void clear() {
        this.keys = new Object[0];
        this.values = new Object[0];
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

class HashMapEntrySet<K, V> extends AbstractSet<Map.Entry<K, V>> {
    HashMap<K, V> backingMap;

    HashMapEntrySet(HashMap<K, V> backingMap) {
        this.backingMap = backingMap;
    }

    public Iterator<Map.Entry<K, V>> iterator() {
        return new HashMapEntrySetIterator<K, V>(this.backingMap);
    }

    public int size() {
        return this.backingMap.size();
    }

    class HashMapEntrySetIterator<K, V> implements Iterator<Map.Entry<K, V>> {
        private int nextEntry;
        private HashMap<K, V> backingMap;

        public HashMapEntrySetIterator(HashMap<K, V> backingMap) {
            this.nextEntry = 0;
            this.backingMap = backingMap;
        }

        public boolean hasNext() {
            return this.nextEntry != this.backingMap.size();
        }

        public Map.Entry<K, V> next() {
            if (!this.hasNext()) {
                throw new NoSuchElementException();
            }

            Map.Entry<K, V> result = new HashMap.Entry<K, V>((K) this.backingMap.keys[nextEntry], (V) this.backingMap.values[nextEntry]);

            this.nextEntry += 1;

            return (Map.Entry<K, V>) result;
        }
    }
}
