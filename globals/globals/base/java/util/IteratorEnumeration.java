package java.util;

class IteratorEnumeration<E> implements Enumeration<E> {
    private Iterator<E> backingIterator;

    IteratorEnumeration(Iterator<E> backingIterator) {
        this.backingIterator = backingIterator;
    }

    public boolean hasMoreElements() {
        return this.backingIterator.hasNext();
    }

    public E nextElement() {
        return this.backingIterator.next();
    }
}
