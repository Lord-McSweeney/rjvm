package java.util;

class ArrayIterator<E> implements Iterator<E> {
    private Object[] data;
    private int index;

    public ArrayIterator(Object[] data) {
        this.data = data;
        this.index = 0;
    }

    public boolean hasNext() {
        return this.index < this.data.length;
    }

    public E next() {
        if (this.index >= this.data.length) {
            throw new NoSuchElementException();
        }

        E element = (E) this.data[this.index];
        this.index += 1;

        return element;
    }
}
