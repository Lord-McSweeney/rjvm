package java.util.concurrent.atomic;

public class AtomicReference<V> {
    public V value;

    public AtomicReference() {
        this.value = null;
    }

    public AtomicReference(V value) {
        this.value = value;
    }
}
