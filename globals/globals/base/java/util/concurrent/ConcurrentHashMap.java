package java.util.concurrent;

import java.util.AbstractMap;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

public class ConcurrentHashMap<K, V> extends AbstractMap<K, V> implements ConcurrentMap<K, V> {
    // TODO make this concurrent-safe

    private HashMap<K, V> backingMap;

    public ConcurrentHashMap(int capacity) {
        this.backingMap = new HashMap<K, V>(capacity);
    }

    public V put(K key, V value) {
        return this.backingMap.put(key, value);
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return this.backingMap.entrySet();
    }
}
