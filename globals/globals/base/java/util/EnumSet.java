package java.util;

public abstract class EnumSet<E extends Enum<E>> extends AbstractSet<E> {
    public static <E extends Enum<E>> EnumSet<E> noneOf(Class<E> elementType) {
        return new ConcreteEnumSet(elementType);
    }

    public static <E extends Enum<E>> EnumSet<E> allOf(Class<E> elementType) {
        ConcreteEnumSet<E> result = (ConcreteEnumSet<E>) EnumSet.noneOf(elementType);

        result.addAllVariants();

        return result;
    }
}

class ConcreteEnumSet<E extends Enum<E>> extends EnumSet<E> {
    // The array of current variants. Variants that aren't present in the set
    // are marked by a `null`.
    Object[] variants;

    // All possible variants.
    Object[] allVariants;

    ConcreteEnumSet(Class<E> elementType) {
        this.allVariants = elementType.getEnumConstants();
        this.variants = new Object[allVariants.length];
    }

    void addAllVariants() {
        this.variants = this.allVariants.clone();
    }

    public int size() {
        int size = 0;

        for (int i = 0; i < this.variants.length; i ++) {
            if (this.variants[i] != null) {
                size += 1;
            }
        }

        return size;
    }

    public Iterator<E> iterator() {
        return new ConcreteEnumSetIterator<E>(this);
    }
}

final class ConcreteEnumSetIterator<E extends Enum<E>> implements Iterator<E> {
    private int nextIndex;
    private ConcreteEnumSet<E> set;

    public ConcreteEnumSetIterator(ConcreteEnumSet<E> set) {
        this.nextIndex = 0;
        this.set = set;

        this.recalculateNext();
    }

    private void recalculateNext() {
        while (true) {
            if (this.nextIndex == this.set.variants.length) {
                // At the end
                return;
            }

            if (this.set.variants[this.nextIndex] != null) {
                // Found the next one
                return;
            }

            this.nextIndex += 1;
        }
    }

    public boolean hasNext() {
        return this.nextIndex != this.set.variants.length;
    }

    public E next() {
        if (!this.hasNext()) {
            throw new NoSuchElementException();
        }

        E result = (E) this.set.variants[this.nextIndex];
        this.nextIndex += 1;
        this.recalculateNext();
        return result;
    }
}
