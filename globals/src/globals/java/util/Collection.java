package java.util;

public interface Collection<E> extends Iterable<E> {
    boolean add(E e);

    boolean addAll(Collection<? extends E> collection);

    void clear();

    int size();

    boolean isEmpty();
}
