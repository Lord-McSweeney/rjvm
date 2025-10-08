package java.util;

public interface Queue<E> extends Collection<E> {
    boolean add(E e);

    E poll();

    E element();

    E peek();
}
