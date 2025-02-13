package java.util;

public interface Collection<E> extends Iterable<E> {
    boolean add(E e);

    void clear();

    int size();

    boolean isEmpty();
}
