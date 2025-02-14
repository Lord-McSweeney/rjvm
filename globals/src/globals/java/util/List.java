package java.util;

public interface List<E> extends Collection<E> {
    boolean add(E e);

    void add(int index, E element);

    E get(int index);

    E set(int index, E element);

    E remove(int index);

    int size();
}
