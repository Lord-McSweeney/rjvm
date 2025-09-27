package java.util;

import rjvm.internal.Todo;

public class Collections {
    public static <K, V> Map<K, V> synchronizedMap(Map<K, V> map) {
        Todo.warnNotImpl("java.util.Collections.synchronizedMap");
        return map;
    }
}
