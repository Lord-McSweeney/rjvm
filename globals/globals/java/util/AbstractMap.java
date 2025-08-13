package java.util;

import rjvm.internal.Todo;

public abstract class AbstractMap<K, V> implements Map<K, V> {
    protected AbstractMap() { }

    public abstract Set<Entry<K, V>> entrySet();

    public Set<K> keySet() {
        Todo.warnNotImpl("java.util.AbstractMap.keySet");

        return null;
    }

    public Collection<V> values() {
        Todo.warnNotImpl("java.util.AbstractMap.values");

        return null;
    }

    public boolean containsKey(K key) {
        Todo.warnNotImpl("java.util.AbstractMap.containsKey");

        return false;
    }

    public V get(K key) {
        Todo.warnNotImpl("java.util.AbstractMap.get");

        return null;
    }

    public V put(K key, V value) {
        Todo.warnNotImpl("java.util.AbstractMap.put");

        return null;
    }

    public V remove(Object key) {
        Todo.warnNotImpl("java.util.AbstractMap.remove");

        return null;
    }

    public void putAll(Map<? extends K, ? extends V> map) {
        Todo.warnNotImpl("java.util.AbstractMap.putAll");
    }

    public int size() {
        return this.entrySet().size();
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }

    public void clear() {
        Todo.warnNotImpl("java.util.AbstractMap.clear");
    }
}
