package java.util;

import rjvm.internal.Todo;

public class ArrayList<E> extends AbstractList<E> implements List<E> {
    Object[] data;
    int size;

    public ArrayList() {
        this(10);
    }

    public ArrayList(Collection<? extends E> collection) {
        this(collection.size());
        this.addAll(collection);
    }

    public ArrayList(int capacity) {
        this.data = new Object[capacity];
        this.size = 0;
    }

    public void ensureCapacity(int minCapacity) {
        if (this.data.length < minCapacity) {
            Object[] newData = new Object[(this.data.length + 1) * 2];

            System.arraycopy(this.data, 0, newData, 0, this.data.length);

            this.data = newData;
        }
    }

    public boolean add(E element) {
        this.ensureCapacity(this.size + 1);

        this.data[this.size] = element;
        this.size += 1;

        return true;
    }

    public void add(int index, E element) {
        this.ensureCapacity(this.size + 1);

        System.arraycopy(this.data, index, this.data, index + 1, this.size - index);
        this.data[index] = element;

        this.size += 1;
    }

    public boolean addAll(Collection<? extends E> collection) {
        return this.addAll(this.size(), collection);
    }

    public boolean addAll(int index, Collection<? extends E> collection) {
        if (index < 0 || index > this.size) {
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
        if (index < 0 || index >= this.size) {
            throw new IndexOutOfBoundsException();
        }

        return (E) this.data[index];
    }

    public E set(int index, E element) {
        if (index < 0 || index >= this.size) {
            throw new IndexOutOfBoundsException();
        }

        E oldElement = (E) this.data[index];

        this.data[index] = element;

        return oldElement;
    }

    public E remove(int index) {
        if (index < 0 || index >= this.size) {
            throw new IndexOutOfBoundsException();
        }

        Object oldElement = this.data[index];
        System.arraycopy(this.data, index + 1, this.data, index, this.size - index - 1);
        this.size -= 1;

        return (E) oldElement;
    }

    public boolean remove(Object search) {
        for (int i = 0; i < this.size; i ++) {
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
        this.size = 0;
    }

    public boolean contains(Object element) {
        for (int i = 0; i < this.size; i ++) {
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
        return new ArrayIterator(this.data, this.size);
    }

    public ListIterator<E> listIterator() {
        return this.listIterator(0);
    }

    public ListIterator<E> listIterator(int index) {
        if (index < 0 || index > this.size) {
            throw new IndexOutOfBoundsException();
        }

        return new ArrayListIterator<E>(this, index);
    }

    public int size() {
        return this.size;
    }

    public boolean isEmpty() {
        return this.size == 0;
    }
}
