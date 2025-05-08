package java.util;

public class ArrayList<E> extends AbstractList<E> implements List<E> {
    Object[] data;

    public ArrayList() {
        this.data = new Object[0];
    }

    public ArrayList(Collection<? extends E> collection) {
        this.data = new Object[0];
        this.addAll(collection);
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

    public boolean addAll(Collection<? extends E> collection) {
        return this.addAll(0, collection);
    }

    public boolean addAll(int index, Collection<? extends E> collection) {
        if (index < 0 || index > this.data.length) {
            throw new IndexOutOfBoundsException();
        }

        // TODO use specialized implementation that grows capacity beforehand
        boolean added = false;

        Iterator<? extends E> iterator = collection.iterator();
        while (iterator.hasNext()) {
            E next = (E) iterator.next();
            this.add(index, next);
            index += 1;
            added = true;
        }

        return added;
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

    public boolean remove(Object search) {
        for (int i = 0; i < this.data.length; i ++) {
            Object element = this.data[i];
            if (element == null) {
                if (search == null) {
                    this.remove(i);
                    return true;
                }
            } else if (element.equals(search)) {
                this.remove(i);
                return true;
            }
        }
        return false;
    }

    public void clear() {
        this.data = new Object[0];
    }

    public boolean contains(Object element) {
        for (int i = 0; i < this.data.length; i ++) {
            if (this.data[i] == null) {
                if (element == null) {
                    return true;
                }
            } else if (this.data[i].equals(element)) {
                return true;
            }
        }

        return false;
    }

    public Iterator<E> iterator() {
        return new ArrayIterator(this.data);
    }

    public ListIterator<E> listIterator() {
        return this.listIterator(0);
    }

    public ListIterator<E> listIterator(int index) {
        if (index < 0 || index > this.data.length) {
            throw new IndexOutOfBoundsException();
        }

        // TODO implement
        return null;
    }

    public int size() {
        return this.data.length;
    }

    public boolean isEmpty() {
        return this.size() == 0;
    }
}
