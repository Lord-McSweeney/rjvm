package java.util;

public interface List<E> extends Collection<E> {
    boolean add(E e);

    void add(int index, E element);

    boolean addAll(Collection<? extends E> collection);

    boolean addAll(int index, Collection<? extends E> collection);

    E get(int index);

    E set(int index, E element);

    E remove(int index);

    int size();

    Object[] toArray();

    <T> T[] toArray(T[] a);
}
