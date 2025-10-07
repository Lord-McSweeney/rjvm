package java.util;

class ArrayIterator<E> implements Iterator<E> {
    private Object[] data;
    private int index;
    private int size;

    public ArrayIterator(Object[] data, int size) {
        this.data = data;
        this.index = 0;
        this.size = size;
    }

    public boolean hasNext() {
        return this.index < this.size;
    }

    public E next() {
        if (this.index >= this.size) {
            throw new NoSuchElementException();
        }

        E element = (E) this.data[this.index];
        this.index += 1;

        return element;
    }
}
