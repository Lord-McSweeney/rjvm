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

    public boolean containsKey(Object key) {
        Iterator<Map.Entry<K, V>> iterator = this.entrySet().iterator();
        while (iterator.hasNext()) {
            Map.Entry<K, V> entry = iterator.next();
            if (key == null) {
                if (entry.getKey() == null) {
                    return true;
                }
            } else if (key.equals(entry.getKey())) {
                return true;
            }
        }

        return false;
    }

    public V get(Object key) {
        Iterator<Map.Entry<K, V>> iterator = this.entrySet().iterator();
        while (iterator.hasNext()) {
            Map.Entry<K, V> entry = iterator.next();
            if (key == null) {
                if (entry.getKey() == null) {
                    return entry.getValue();
                }
            } else if (key.equals(entry.getKey())) {
                return entry.getValue();
            }
        }

        return null;
    }

    public V put(K key, V value) {
        throw new UnsupportedOperationException();
    }

    public V remove(Object key) {
        Todo.warnNotImpl("java.util.AbstractMap.remove");

        return null;
    }

    public void putAll(Map<? extends K, ? extends V> map) {
        for (Map.Entry<? extends K, ? extends V> entry : map.entrySet()) {
            this.put(entry.getKey(), entry.getValue());
        }
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
