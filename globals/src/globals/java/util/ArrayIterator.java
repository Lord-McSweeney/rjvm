package java.util;

class ArrayIterator<E> implements Iterator<E> {
    private Object[] data;
    private int index;

    public ArrayIterator(Object[] data) {
        this.data = data;
        this.index = 0;
    }

    public boolean hasNext() {
        return this.data.length < this.index;
    }

    public E next() {
        if (this.data.length >= this.index) {
            throw new NoSuchElementException();
        }

        E element = (E) this.data[this.index];
        this.index += 1;

        return element;
    }
}
