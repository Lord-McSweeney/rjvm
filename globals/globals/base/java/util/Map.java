package java.util;

public interface Map<K, V> {
    V get(Object key);

    V put(K key, V value);

    void clear();

    Set<Map.Entry<K, V>> entrySet();

    interface Entry<K, V> {
        K getKey();

        V getValue();

        V setValue(V value);
    }
}
