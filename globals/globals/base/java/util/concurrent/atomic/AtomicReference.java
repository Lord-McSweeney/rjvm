package java.util.concurrent.atomic;

public class AtomicReference<V> {
    public V value;

    public AtomicReference() {
        this.value = null;
    }

    public AtomicReference(V value) {
        this.value = value;
    }

    public final V get() {
        return this.value;
    }

    public final void set(V value) {
        this.value = value;
    }

    public final boolean compareAndSet(V expect, V update) {
        if (this.value == expect) {
            this.value = update;
            return true;
        } else {
            return false;
        }
    }
}
