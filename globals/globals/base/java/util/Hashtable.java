package java.util;

import rjvm.internal.Todo;

public class Hashtable<K, V> extends Dictionary<K, V> implements Map<K, V> {
    HashMap<K, V> backingMap;

    public Hashtable() {
        this.backingMap = new HashMap<K, V>();
    }

    public synchronized Enumeration<K> keys() {
        Todo.warnNotImpl("java.util.Hashtable.keys");

        return null;
    }

    public synchronized Enumeration<V> elements() {
        Todo.warnNotImpl("java.util.Hashtable.elements");

        return null;
    }

    public synchronized V get(Object key) {
        if (key == null) {
            throw new NullPointerException();
        }

        return this.backingMap.get(key);
    }

    public synchronized V put(K key, V value) {
        if (key == null || value == null) {
            throw new NullPointerException();
        }

        return this.backingMap.put(key, value);
    }

    public synchronized boolean containsKey(Object key) {
        if (key == null) {
            throw new NullPointerException();
        }

        return this.backingMap.containsKey(key);
    }

    public synchronized V remove(Object key) {
        if (key == null) {
            throw new NullPointerException();
        }

        return this.backingMap.remove(key);
    }

    public synchronized void putAll(Map<? extends K, ? extends V> map) {
        this.backingMap.putAll(map);
    }

    // NOTE: Not synchronized
    public Set<Map.Entry<K, V>> entrySet() {
        return this.backingMap.entrySet();
    }

    public synchronized int size() {
        return this.backingMap.size();
    }

    public synchronized boolean isEmpty() {
        return this.backingMap.isEmpty();
    }

    public synchronized void clear() {
        this.backingMap.clear();
    }
}
