package java.util;

public class Vector<E> extends AbstractList<E> {
    Object[] data;

    public Vector() {
        this.data = new Object[0];
    }

    public boolean add(E element) {
        this.add(this.size(), element);
        return true;
    }

    public void add(int index, E element) {
        Object[] newData = new Object[this.data.length + 1];

        System.arraycopy(this.data, 0, newData, 0, index);
        newData[index] = element;
        System.arraycopy(this.data, index, newData, index + 1, this.size() - index);

        this.data = newData;
    }

    public E get(int index) {
        if (index < 0 || index >= this.data.length) {
            throw new IndexOutOfBoundsException();
        }

        return (E) this.data[index];
    }

    public E set(int index, E element) {
        if (index < 0 || index >= this.data.length) {
            throw new IndexOutOfBoundsException();
        }

        E oldElement = (E) this.data[index];

        this.data[index] = element;

        return oldElement;
    }

    public E remove(int index) {
        if (index < 0 || index >= this.data.length) {
            throw new IndexOutOfBoundsException();
        }

        E oldElement = (E) this.data[index];

        Object[] newData = new Object[this.data.length - 1];

        System.arraycopy(this.data, 0, newData, 0, index);
        System.arraycopy(this.data, index + 1, newData, index, this.size() - index - 1);

        this.data = newData;

        return oldElement;
    }

    public void clear() {
        this.data = new Object[0];
    }

    public int size() {
        return this.data.length;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
