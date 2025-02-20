package java.util;

public abstract class AbstractMap<K, V> implements Map<K, V> {
    protected AbstractMap() { }

    public abstract Set<Entry<K, V>> entrySet();

    public Set<K> keySet() {
        // TODO implement
        return null;
    }

    public Collection<V> values() {
        // TODO implement
        return null;
    }

    public boolean containsKey(K key) {
        // TODO implement
        return false;
    }

    public V get(K key) {
        // TODO implement
        return null;
    }

    public V put(K key, V value) {
        // TODO implement
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
        return this.entrySet().size();
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }

    public void clear() {
        // TODO implement
    }
}
