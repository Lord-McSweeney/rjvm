package java.util;

import rjvm.internal.Todo;

public class Collections {
    public static final List EMPTY_LIST;
    public static final Map EMPTY_MAP;
    public static final Set EMPTY_SET;

    private static final Iterator EMPTY_ITERATOR;

    static {
        EMPTY_LIST = new EmptyList();
        EMPTY_MAP = new EmptyMap();
        EMPTY_SET = new EmptySet();
        EMPTY_ITERATOR = new EmptyIterator();
    }

    public static <K, V> Map<K, V> synchronizedMap(Map<K, V> map) {
        Todo.warnNotImpl("java.util.Collections.synchronizedMap");
        return map;
    }

    public static final <T> List<T> emptyList() {
        return EMPTY_LIST;
    }

    public static final <K, V> Map<K, V> emptyMap() {
        return EMPTY_MAP;
    }

    public static final <E> Set<E> emptySet() {
        return EMPTY_SET;
    }

    public static <T> Iterator<T> emptyIterator() {
        return EMPTY_ITERATOR;
    }

    public static <K, V> Map<K, V> unmodifiableMap(Map<? extends K, ? extends V> map) {
        // FIXME get the generics working right
        return new ImmutableMap<K, V>((Map<K, V>) map);
    }
}

class EmptyList<E> implements List<E> {
    public boolean add(E e) {
        return false;
    }

    public void add(int index, E element) { }

    public boolean addAll(Collection<? extends E> collection) {
        return false;
    }

    public boolean addAll(int index, Collection<? extends E> collection) {
        return false;
    }

    public void clear() { }

    public Iterator<E> iterator() {
        return Collections.emptyIterator();
    }

    public boolean isEmpty() {
        return true;
    }

    public E get(int index) {
        throw new IndexOutOfBoundsException();
    }

    public E set(int index, E element) {
        throw new IndexOutOfBoundsException();
    }

    public E remove(int index) {
        throw new IndexOutOfBoundsException();
    }

    public int size() {
        return 0;
    }

    public List<E> subList(int fromIndex, int toIndex) {
        if (fromIndex == 0 && (toIndex == 0 || toIndex == 1)) {
            return this;
        } else {
            throw new IndexOutOfBoundsException();
        }
    }

    public Object[] toArray() {
        return new Object[0];
    }

    public <T> T[] toArray(T[] a) {
        // Element after the last is set to null
        if (a.length > 0) {
            a[0] = null;
        }

        return a;
    }
}

class EmptyMap<K, V> implements Map<K, V> {
    EmptyMap() { }

    public V get(Object key) {
        return null;
    }

    public V put(K key, V value) {
        throw new UnsupportedOperationException();
    }

    public void clear() {
        throw new UnsupportedOperationException();
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return Collections.emptySet();
    }
}

class EmptySet<E> implements Set<E> {
    public boolean add(E e) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(Collection<? extends E> collection) {
        throw new UnsupportedOperationException();
    }

    public void clear() {
        throw new UnsupportedOperationException();
    }

    public Iterator<E> iterator() {
        return Collections.emptyIterator();
    }

    public int size() {
        return 0;
    }

    public boolean isEmpty() {
        return true;
    }

    public Object[] toArray() {
        return new Object[0];
    }

    public <T> T[] toArray(T[] a) {
        // Element after the last is set to null
        if (a.length > 0) {
            a[0] = null;
        }

        return a;
    }
}

class EmptyIterator<E> implements Iterator<E> {
    public boolean hasNext() {
        return false;
    }

    public E next() {
        throw new NoSuchElementException();
    }
}

class ImmutableMap<K, V> implements Map<K, V> {
    private Map<K, V> backingMap;

    ImmutableMap(Map<K, V> backingMap) {
        this.backingMap = backingMap;
    }

    public V get(Object key) {
        return this.backingMap.get(key);
    }

    public V put(K key, V value) {
        throw new UnsupportedOperationException();
    }

    public void clear() {
        throw new UnsupportedOperationException();
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return this.backingMap.entrySet();
    }
}
