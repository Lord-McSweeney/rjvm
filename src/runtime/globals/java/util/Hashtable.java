package java.util;

public class Hashtable<K, V> extends Dictionary<K, V> implements Map<K, V> {
    // FIXME we should really not be reimplementing all the AbstractMap functions...
    // TODO actually implement a hashmap

    Object[] keys;
    Object[] values;

    public Hashtable() {
        this.keys = new Object[0];
        this.values = new Object[0];
    }

    public Enumeration<K> keys() {
        // TODO implement
        return null;
    }

    public Enumeration<V> elements() {
        // TODO implement
        return null;
    }

    public V get(Object key) {
        if (key == null) {
            throw new NullPointerException();
        }

        for (int i = 0; i < this.keys.length; i ++) {
            if (this.keys[i].equals(key)) {
                return (V) this.values[i];
            }
        }

        return null;
    }

    public V put(K key, V value) {
        if (key == null || value == null) {
            throw new NullPointerException();
        }

        for (int i = 0; i < this.keys.length; i ++) {
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
        // TODO implement
        return null;
    }

    public void putAll(Map<? extends K, ? extends V> map) {
        // TODO implement
    }

    public int size() {
        return this.keys.length;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }

    public void clear() {
        this.keys = new Object[0];
        this.values = new Object[0];
    }
}
