package java.util;

import rjvm.internal.Todo;

public class HashMap<K, V> extends AbstractMap<K, V> {
    // FIXME we should really not be reimplementing all the AbstractMap functions...
    // TODO actually implement a hashmap

    HashMap.Entry[] data;

    public HashMap() {
        this.data = new HashMap.Entry[0];
    }

    public HashMap(int capacity) {
        // TODO implement capacity
        this.data = new HashMap.Entry[0];
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return new HashMapEntrySet<K, V>(this);
    }

    public Set<K> keySet() {
        return new HashMapKeySet<K, V>(this);
    }

    public Collection<V> values() {
        ArrayList<V> values = new ArrayList<V>();
        for (int i = 0; i < this.data.length; i ++) {
            values.add((V) this.data[i].value);
        }

        // TODO this should be an interactive `Collection`
        return values;
    }

    public boolean containsKey(K key) {
        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i].key == null) {
                if (key == null) {
                    return true;
                } else {
                    continue;
                }
            }

            if (this.data[i].key.equals(key)) {
                return true;
            }
        }

        return false;
    }

    public V get(Object key) {
        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i].key == null) {
                if (key == null) {
                    return (V) this.data[i].value;
                } else {
                    continue;
                }
            }

            if (this.data[i].key.equals(key)) {
                return (V) this.data[i].value;
            }
        }

        return null;
    }

    public V put(K key, V value) {
        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i].key == null) {
                if (key == null) {
                    // The key was present, so replace the old value and return it
                    Object oldValue = this.data[i].value;
                    this.data[i].value = value;
                    return (V) oldValue;
                } else {
                    continue;
                }
            }

            if (this.data[i].key.equals(key)) {
                // The key was present, so replace the old value and return it
                Object oldValue = this.data[i].value;
                this.data[i].value = value;
                return (V) oldValue;
            }
        }

        // The key wasn't present, so resize the array, insert the
        // key and value at the end, and return null.
        int newSize = this.size() + 1;
        Entry[] newData = new Entry[newSize];

        System.arraycopy(this.data, 0, newData, 0, this.data.length);

        newData[newSize - 1] = new Entry<K, V>(key, value);

        this.data = newData;

        return null;
    }

    public V remove(Object key) {
        int pos = -1;
        Object previousValue = null;

        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i].key == null) {
                if (key == null) {
                    // The key was present, so record the index
                    pos = i;
                    previousValue = this.data[i].value;
                    break;
                } else {
                    continue;
                }
            }

            if (this.data[i].key.equals(key)) {
                pos = i;
                previousValue = this.data[i].value;
                break;
            }
        }

        if (pos == -1) {
            return null;
        }
        // The key was present, so resize the array, remove the
        // key and value at the end, and return the previous value.
        int newSize = this.size() - 1;
        Entry[] newData = new Entry[newSize];

        System.arraycopy(this.data, 0, newData, 0, pos);
        if (pos != newSize) {
            System.arraycopy(this.data, pos + 1, newData, pos + 1, newSize - pos);
        }

        this.data = newData;

        return (V) previousValue;
    }

    public void putAll(Map<? extends K, ? extends V> map) {
        Todo.warnNotImpl("java.util.HashMap.putAll");
    }

    public int size() {
        return this.data.length;
    }

    public void clear() {
        this.data = new Entry[0];
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

        public V setValue(V value) {
            V oldValue = this.value;
            this.value = value;
            return oldValue;
        }
    }
}

abstract class HashMapGenericIterator<K, V, E> implements Iterator<E> {
    private int nextEntry;
    private HashMap<K, V> backingMap;

    public HashMapGenericIterator(HashMap<K, V> backingMap) {
        this.nextEntry = 0;
        this.backingMap = backingMap;
    }

    public boolean hasNext() {
        return this.nextEntry != this.backingMap.size();
    }

    public abstract E next();

    // The class that overrides this class should use this function to get
    // the next entry.
    protected final HashMap.Entry<K, V> nextEntry() {
        if (!this.hasNext()) {
            throw new NoSuchElementException();
        }

        HashMap.Entry<K, V> result = (HashMap.Entry<K, V>) this.backingMap.data[nextEntry];

        this.nextEntry += 1;

        return result;
    }
}

class HashMapEntrySet<K, V> extends AbstractSet<Map.Entry<K, V>> {
    private HashMap<K, V> backingMap;

    HashMapEntrySet(HashMap<K, V> backingMap) {
        this.backingMap = backingMap;
    }

    class HashMapEntrySetIterator<K, V> extends HashMapGenericIterator<K, V, Map.Entry<K, V>> {
        public HashMapEntrySetIterator(HashMap<K, V> backingMap) {
            super(backingMap);
        }

        public Map.Entry<K, V> next() {
            return super.nextEntry();
        }
    }

    public Iterator<Map.Entry<K, V>> iterator() {
        return new HashMapEntrySetIterator<K, V>(this.backingMap);
    }

    public void clear() {
        this.backingMap.clear();
    }

    public int size() {
        return this.backingMap.size();
    }
}

class HashMapKeySet<K, V> extends AbstractSet<K> {
    private HashMap<K, V> backingMap;

    HashMapKeySet(HashMap<K, V> backingMap) {
        this.backingMap = backingMap;
    }

    class HashMapKeyIterator<K, V> extends HashMapGenericIterator<K, V, K> {
        public HashMapKeyIterator(HashMap<K, V> backingMap) {
            super(backingMap);
        }

        public K next() {
            return super.nextEntry().key;
        }
    }

    public boolean contains(Object element) {
        return this.backingMap.containsKey((K) element);
    }

    public Iterator<K> iterator() {
        return new HashMapKeyIterator<K, V>(this.backingMap);
    }

    public void clear() {
        this.backingMap.clear();
    }

    public int size() {
        return this.backingMap.size();
    }
}
