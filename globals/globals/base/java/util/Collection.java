package java.util;

public interface Collection<E> extends Iterable<E> {
    boolean add(E e);

    boolean addAll(Collection<? extends E> collection);

    boolean contains(Object o);

    void clear();

    int size();

    boolean isEmpty();

    Object[] toArray();

    <T> T[] toArray(T[] a);
}
