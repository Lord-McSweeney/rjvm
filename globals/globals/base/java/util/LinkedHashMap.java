package java.util;

import rjvm.internal.Todo;

public class LinkedHashMap<K, V> extends HashMap<K, V> implements Map<K, V> {
    public Set<Map.Entry<K, V>> entrySet() {
        Todo.warnNotImpl("java.util.LinkedHashMap.entrySet");

        return null;
    }
}
