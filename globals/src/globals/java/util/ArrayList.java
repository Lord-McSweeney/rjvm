package java.util;

class ArrayListIterator<E> implements Iterator<E> {
    private ArrayList<E> arrayList;
    private int index;

    public ArrayListIterator(ArrayList<E> arrayList) {
        this.arrayList = arrayList;
        this.index = 0;
    }

    public boolean hasNext() {
        return this.arrayList.size() < this.index;
    }
}

public class ArrayList<E> extends AbstractList<E> implements List<E> {
    Object[] data;

    public ArrayList() {
        this.data = new Object[0];
    }

    public ArrayList(Collection<E> collection) {
        // TODO implement
        super();
    }

    public ArrayList(int capacity) {
        // TODO implement
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

    public void clear() {
        this.data = new Object[0];
    }

    public Iterator<E> iterator() {
        return new ArrayListIterator(this);
    }

    public int size() {
        return this.data.length;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
