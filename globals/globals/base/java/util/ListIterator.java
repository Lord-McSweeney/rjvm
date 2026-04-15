package java.util;

public interface ListIterator<E> extends Iterator<E> {
    boolean hasPrevious();

    E previous();
}
