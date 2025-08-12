package java.util.concurrent;

import rjvm.internal.Todo;

import java.util.AbstractMap;
import java.util.Set;

public class ConcurrentHashMap<K, V> extends AbstractMap<K, V> implements ConcurrentMap<K, V> {
    public Set<Entry<K, V>> entrySet() {
        Todo.warnNotImpl("java.util.concurrent.ConcurrentHashMap.entrySet");

        return null;
    }

    public ConcurrentHashMap(int capacity) {
        super();

        // TODO implement
    }
}
