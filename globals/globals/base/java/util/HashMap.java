package java.util;

import rjvm.internal.Todo;

public class HashMap<K, V> extends AbstractMap<K, V> {
    // FIXME we should really not be reimplementing all the AbstractMap functions...

    // TODO make it possible to dynamically increase bucket count
    private static final int BUCKET_COUNT = 8;

    HashMap.Entry[] data;
    private int bucketSizeLog2;
    private int size;

    public HashMap() {
        this(16);
    }

    public HashMap(int capacity) {
        // TODO implement capacity

        this.data = new HashMap.Entry[BUCKET_COUNT];
        this.bucketSizeLog2 = 0;
        this.size = 0;
    }

    public V get(Object key) {
        int code = HashMap.hashCode(key) & (BUCKET_COUNT - 1);
        int bucketStartIndex = code << this.bucketSizeLog2;
        int bucketEndIndex = bucketStartIndex + (1 << this.bucketSizeLog2);

        for (int i = bucketStartIndex; i < bucketEndIndex; i ++) {
            HashMap.Entry<K, V> thisEntry = (Entry<K, V>) this.data[i];
            if (thisEntry != null && HashMap.equals(thisEntry.key, key)) {
                return thisEntry.value;
            }
        }
        return null;
    }

    public V put(K key, V value) {
        int code = HashMap.hashCode(key) & (BUCKET_COUNT - 1);
        int bucketStartIndex = code << this.bucketSizeLog2;
        int bucketEndIndex = bucketStartIndex + (1 << this.bucketSizeLog2);

        // Try to find the key in the bucket
        for (int i = bucketStartIndex; i < bucketEndIndex; i ++) {
            HashMap.Entry<K, V> originalEntry = (Entry<K, V>) this.data[i];
            if (originalEntry != null && HashMap.equals(originalEntry.key, key)) {
                // Replace
                this.data[i] = new HashMap.Entry<K, V>(key, value);
                return originalEntry.value;
            }
        }

        // Try to find an empty position
        for (int i = bucketStartIndex; i < bucketEndIndex; i ++) {
            if (this.data[i] == null) {
                // Insert
                this.data[i] = new HashMap.Entry<K, V>(key, value);
                this.size += 1;
                return null;
            }
        }

        // Failed to add: resize and try again
        int newBucketSizeLog2 = this.bucketSizeLog2 + 1;
        HashMap.Entry[] newData = new HashMap.Entry[this.data.length * 2];
        for (int i = 0; i < BUCKET_COUNT; i ++) {
            for (int j = 0; j < (1 << this.bucketSizeLog2); j ++) {
                int oldIndex = (i << this.bucketSizeLog2) + j;
                int newIndex = (i << newBucketSizeLog2) + j;
                newData[newIndex] = this.data[oldIndex];
            }
        }
        this.bucketSizeLog2 = newBucketSizeLog2;
        this.data = newData;

        // Reload indices
        bucketStartIndex = code << this.bucketSizeLog2;
        bucketEndIndex = bucketStartIndex + (1 << this.bucketSizeLog2);

        // Try again. We don't need to check for replacing this time, only
        // inserting
        for (int i = bucketStartIndex; i < bucketEndIndex; i ++) {
            if (this.data[i] == null) {
                this.data[i] = new HashMap.Entry<K, V>(key, value);
                this.size += 1;
                return null;
            }
        }

        throw new Error("unreachable");
    }

    public boolean containsKey(Object key) {
        int code = HashMap.hashCode(key) & (BUCKET_COUNT - 1);
        int bucketStartIndex = code << this.bucketSizeLog2;
        int bucketEndIndex = bucketStartIndex + (1 << this.bucketSizeLog2);

        for (int i = bucketStartIndex; i < bucketEndIndex; i ++) {
            HashMap.Entry<K, V> thisEntry = (Entry<K, V>) this.data[i];
            if (thisEntry != null && HashMap.equals(thisEntry.key, key)) {
                return true;
            }
        }

        return false;
    }

    public V remove(Object key) {
        int code = HashMap.hashCode(key) & (BUCKET_COUNT - 1);
        int bucketStartIndex = code << this.bucketSizeLog2;
        int bucketEndIndex = bucketStartIndex + (1 << this.bucketSizeLog2);

        for (int i = bucketStartIndex; i < bucketEndIndex; i ++) {
            HashMap.Entry<K, V> thisEntry = (Entry<K, V>) this.data[i];
            if (thisEntry != null && HashMap.equals(thisEntry.key, key)) {
                this.data[i] = null;
                this.size -= 1;
                return thisEntry.value;
            }
        }
        return null;
    }

    public void putAll(Map<? extends K, ? extends V> map) {
        for (Map.Entry<? extends K, ? extends V> entry : map.entrySet()) {
            this.put(entry.getKey(), entry.getValue());
        }
    }

    public void clear() {
        // TODO maybe we don't need to fully reset everything? Could we just
        // null out the buckets?
        this.data = new HashMap.Entry[BUCKET_COUNT];
        this.bucketSizeLog2 = 0;
        this.size = 0;
    }

    public int size() {
        return this.size;
    }

    // Useful functions
    private static int hashCode(Object o) {
        if (o == null) {
            return 0;
        } else {
            return o.hashCode();
        }
    }

    private static boolean equals(Object o1, Object o2) {
        if (o1 == null) {
            return o2 == null;
        } else {
            return o1.equals(o2);
        }
    }

    public Set<Map.Entry<K, V>> entrySet() {
        return new HashMapEntrySet<K, V>(this);
    }

    public Set<K> keySet() {
        return new HashMapKeySet<K, V>(this);
    }

    public Collection<V> values() {
        ArrayList<V> values = new ArrayList<V>();
        for (int i = 0; i < this.data.length; i ++) {
            values.add((V) this.data[i].value);
        }

        // TODO this should be an interactive `Collection`
        return values;
    }

    static class Entry<K, V> implements Map.Entry<K, V> {
        K key;
        V value;

        Entry<K, V> left;
        Entry<K, V> right;

        Entry<K,V> parent;

        Entry(K key, V value) {
            this.key = key;
            this.value = value;
        }

        public K getKey() {
            return this.key;
        }

        public V getValue() {
            return this.value;
        }

        public V setValue(V value) {
            V oldValue = this.value;
            this.value = value;
            return oldValue;
        }
    }
}

abstract class HashMapGenericIterator<K, V, E> implements Iterator<E> {
    private int nextEntry;
    private HashMap<K, V> backingMap;

    public HashMapGenericIterator(HashMap<K, V> backingMap) {
        this.backingMap = backingMap;

        this.nextEntry = 0;
        this.findNextEntry();
    }

    private void findNextEntry() {
        // -1 means out of elements
        if (this.nextEntry == -1) {
            return;
        }

        HashMap.Entry[] data = this.backingMap.data;

        while (true) {
            if (this.nextEntry == data.length) {
                this.nextEntry = -1;
                break;
            }

            if (data[this.nextEntry] != null) {
                // Found a valid entry
                break;
            }

            this.nextEntry += 1;
        }
    }

    public boolean hasNext() {
        return this.nextEntry != -1;
    }

    public abstract E next();

    // The class that overrides this class should use this function to get
    // the next entry.
    protected final HashMap.Entry<K, V> nextEntry() {
        if (!this.hasNext()) {
            throw new NoSuchElementException();
        }

        HashMap.Entry<K, V> result = (HashMap.Entry<K, V>) this.backingMap.data[nextEntry];

        this.nextEntry += 1;
        this.findNextEntry();

        return result;
    }
}

class HashMapEntrySet<K, V> extends AbstractSet<Map.Entry<K, V>> {
    private HashMap<K, V> backingMap;

    HashMapEntrySet(HashMap<K, V> backingMap) {
        this.backingMap = backingMap;
    }

    class HashMapEntrySetIterator<K, V> extends HashMapGenericIterator<K, V, Map.Entry<K, V>> {
        public HashMapEntrySetIterator(HashMap<K, V> backingMap) {
            super(backingMap);
        }

        public Map.Entry<K, V> next() {
            return super.nextEntry();
        }
    }

    public Iterator<Map.Entry<K, V>> iterator() {
        return new HashMapEntrySetIterator<K, V>(this.backingMap);
    }

    public void clear() {
        this.backingMap.clear();
    }

    public int size() {
        return this.backingMap.size();
    }
}

class HashMapKeySet<K, V> extends AbstractSet<K> {
    private HashMap<K, V> backingMap;

    HashMapKeySet(HashMap<K, V> backingMap) {
        this.backingMap = backingMap;
    }

    class HashMapKeyIterator<K, V> extends HashMapGenericIterator<K, V, K> {
        public HashMapKeyIterator(HashMap<K, V> backingMap) {
            super(backingMap);
        }

        public K next() {
            return super.nextEntry().key;
        }
    }

    public boolean contains(Object element) {
        return this.backingMap.containsKey((K) element);
    }

    public Iterator<K> iterator() {
        return new HashMapKeyIterator<K, V>(this.backingMap);
    }

    public void clear() {
        this.backingMap.clear();
    }

    public int size() {
        return this.backingMap.size();
    }
}
