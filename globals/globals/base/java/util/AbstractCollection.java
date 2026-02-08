package java.util;

import java.lang.reflect.Array;

public abstract class AbstractCollection<E> implements Collection<E> {
    protected AbstractCollection() { }

    public boolean add(E element) {
        throw new UnsupportedOperationException();
    }

    public boolean addAll(Collection<? extends E> collection) {
        boolean added = false;

        Iterator<? extends E> iterator = collection.iterator();
        while (iterator.hasNext()) {
            E next = (E) iterator.next();
            this.add(next);
            added = true;
        }

        return added;
    }

    public boolean contains(Object o) {
        Iterator<E> iterator = this.iterator();
        while (iterator.hasNext()) {
            E current = iterator.next();
            if (o == null) {
                if (current == null) {
                    return true;
                }
            } else if (o.equals(current)) {
                return true;
            }
        }

        return false;
    }

    public void clear() {
        // TODO implement
    }

    public abstract Iterator<E> iterator();

    public abstract int size();

    public boolean isEmpty() {
        return this.size() == 0;
    }

    public String toString() {
        StringBuilder builder = new StringBuilder();
        builder.append('[');

        Iterator<E> iter = this.iterator();
        while (iter.hasNext()) {
            E next = iter.next();
            builder.append(next.toString());

            // Append comma only if there's a next element
            if (iter.hasNext()) {
                builder.append(", ");
            }
        }

        builder.append(']');

        return builder.toString();
    }

    public Object[] toArray() {
        // FIXME handle case when iterator gives more or less elements than size
        Object[] array = new Object[this.size()];

        Iterator<E> iterator = this.iterator();
        for (int i = 0; i < array.length; i ++) {
            array[i] = iterator.next();
        }

        return array;
    }

    public <T> T[] toArray(T[] passedArray) {
        // FIXME handle case when iterator gives more or less elements than size
        // FIXME optimize for perf?
        Object[] array = new Object[this.size()];

        Iterator<E> iterator = this.iterator();
        for (int i = 0; i < array.length; i ++) {
            array[i] = iterator.next();
        }

        if (array.length == passedArray.length) {
            System.arraycopy(array, 0, passedArray, 0, array.length);
            return passedArray;
        } else if (array.length < passedArray.length) {
            System.arraycopy(array, 0, passedArray, 0, array.length);
            passedArray[array.length] = null;
            return passedArray;
        } else {
            Class componentType = passedArray.getClass().getComponentType();
            T[] newPassedArray = (T[]) Array.newInstance(componentType, array.length);

            System.arraycopy(array, 0, newPassedArray, 0, newPassedArray.length);
            return newPassedArray;
        }
    }
}
