package java.util;

class ArrayListIterator<E> implements ListIterator<E> {
    private ArrayList<E> data;
    private int index;

    public ArrayListIterator(ArrayList<E> data, int index) {
        this.data = data;
        this.index = index;
    }

    public boolean hasPrevious() {
        return this.index != 0;
    }

    public boolean hasNext() {
        return this.index < this.data.size();
    }

    public E previous() {
        if (this.index == 0) {
            throw new NoSuchElementException();
        }

        this.index -= 1;
        E element = this.data.get(this.index);

        return element;
    }

    public E next() {
        if (this.index >= this.data.size()) {
            throw new NoSuchElementException();
        }

        E element = this.data.get(this.index);
        this.index += 1;

        return element;
    }
}
