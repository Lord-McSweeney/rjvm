package java.util;

public class Vector<E> extends AbstractList<E> {
    // TODO capacity
    Object[] data;

    public Vector() {
        this.data = new Object[0];
    }

    public synchronized boolean add(E element) {
        Object[] newData = new Object[this.data.length + 1];

        System.arraycopy(this.data, 0, newData, 0, this.data.length);
        newData[this.data.length] = element;

        this.data = newData;

        return true;
    }

    public synchronized void add(int index, E element) {
        Object[] newData = new Object[this.data.length + 1];

        System.arraycopy(this.data, 0, newData, 0, index);
        newData[index] = element;
        System.arraycopy(this.data, index, newData, index + 1, this.data.length - index);

        this.data = newData;
    }

    public synchronized E get(int index) {
        if (index < 0 || index >= this.data.length) {
            throw new ArrayIndexOutOfBoundsException();
        }

        return (E) this.data[index];
    }

    public synchronized E set(int index, E element) {
        if (index < 0 || index >= this.data.length) {
            throw new ArrayIndexOutOfBoundsException();
        }

        E oldElement = (E) this.data[index];

        this.data[index] = element;

        return oldElement;
    }

    public synchronized E remove(int index) {
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

    public synchronized int size() {
        return this.data.length;
    }

    public synchronized boolean isEmpty() {
        return this.data.length == 0;
    }

    // Legacy methods

    public synchronized E elementAt(int index) {
        if (index < 0 || index >= this.data.length) {
            throw new ArrayIndexOutOfBoundsException();
        }

        return (E) this.data[index];
    }

    public synchronized void addElement(E element) {
        Object[] newData = new Object[this.data.length + 1];

        System.arraycopy(this.data, 0, newData, 0, this.data.length);
        newData[this.data.length] = element;

        this.data = newData;
    }
}
