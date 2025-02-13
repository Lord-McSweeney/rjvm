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
    public ArrayList() {
        super();
    }

    public ArrayList(Collection<E> collection) {
        super();
    }

    public ArrayList(int capacity) {
        super();
    }

    public boolean add(E element) {
        this.add(this.size(), element);
        return true;
    }

    public void add(int index, E element) {
        // TODO implement
    }

    public E get(int index) {
        // TODO implement
        return null;
    }

    public void clear() {
        // TODO implement
    }

    public Iterator<E> iterator() {
        // TODO implement
        return new ArrayListIterator(this);
    }

    public int size() {
        // TODO implement
        return 0;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
