package java.util;

import rjvm.internal.Todo;

public class HashMap<K, V> extends AbstractMap<K, V> {
    // FIXME we should really not be reimplementing all the AbstractMap functions...
    // TODO actually implement a hashmap

    Object[] keys;
    Object[] values;

    public HashMap() {
        super();

        this.keys = new Object[0];
        this.values = new Object[0];
    }

    public HashMap(int capacity) {
        super();

        this.keys = new Object[0];
        this.values = new Object[0];
    }

    public Set<Entry<K, V>> entrySet() {
        Todo.warnNotImpl("java.util.HashMap.entrySet");

        return null;
    }

    public Collection<V> values() {
        Todo.warnNotImpl("java.util.HashMap.values");

        return null;
    }

    public boolean containsKey(K key) {
        Todo.warnNotImpl("java.util.HashMap.containsKey");

        return false;
    }

    public V get(K key) {
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
        Todo.warnNotImpl("java.util.HashMap.remove");

        return null;
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
}
