package java.util;

public abstract class AbstractList<E> extends AbstractCollection<E> implements List<E> {
    protected AbstractList() { }

    public boolean add(E element) {
        this.add(this.size(), element);
        return true;
    }

    public void add(int index, E element) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(int index, Collection<? extends E> collection) {
        boolean added = false;
        int curIndex = index;

        Iterator<? extends E> iterator = collection.iterator();
        while (iterator.hasNext()) {
            E next = (E) iterator.next();
            this.add(index, next);

            added = true;
            index += 1;
        }

        return added;
    }

    public abstract E get(int index);

    public E set(int index, E element) {
        throw new UnsupportedOperationException();
    }

    public E remove(int index) {
        throw new UnsupportedOperationException();
    }

    public void clear() {
        // TODO implement
    }

    public List<E> subList(int fromIndex, int toIndex) {
        if (fromIndex < 0 || toIndex > this.size()) {
            throw new IndexOutOfBoundsException();
        }
        if (fromIndex > toIndex) {
            throw new IllegalArgumentException();
        }

        int size = toIndex - fromIndex;
        return new SubList<E>(this, fromIndex, size);
    }

    public Iterator<E> iterator() {
        return new AbstractListIterator<E>(this);
    }
}

class SubList<E> extends AbstractList<E> {
    private AbstractList<E> backingList;
    private int start;
    private int size;

    SubList(AbstractList<E> backingList, int start, int size) {
        this.backingList = backingList;
        this.start = start;
        this.size = size;
    }

    public E get(int index) {
        return this.backingList.get(this.start + index);
    }

    public E set(int index, E element) {
        return this.backingList.set(this.start + index, element);
    }

    // Other methods are TODO

    public int size() {
        return this.size;
    }
}

class AbstractListIterator<E> implements Iterator<E> {
    private AbstractList<E> backingList;
    private int currentIndex;

    AbstractListIterator(AbstractList<E> backingList) {
        this.backingList = backingList;
        this.currentIndex = 0;
    }

    public boolean hasNext() {
        return this.currentIndex < this.backingList.size();
    }

    public E next() {
        return this.backingList.get(this.currentIndex ++);
    }
}
