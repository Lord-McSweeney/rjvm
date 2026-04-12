package java.util;

import java.lang.reflect.Array;

public class Arrays {
    private Arrays() {}

    public static <E> List<E> asList(E... arr) {
        return new ArrayBackedList<E>(arr);
    }

    public static <T> T[] copyOf(T[] original, int newLength) {
        // Create a new array of the correct type
        Class componentType = original.getClass().getComponentType();
        T[] newArray = (T[]) Array.newInstance(componentType, newLength);

        int usedLength;
        if (newLength > original.length) {
            usedLength = original.length;
        } else {
            usedLength = newLength;
        }

        System.arraycopy(original, 0, newArray, 0, usedLength);
        return newArray;
    }

    public static int[] copyOf(int[] original, int newLength) {
        int[] newArray = new int[newLength];

        int usedLength;
        if (newLength > original.length) {
            usedLength = original.length;
        } else {
            usedLength = newLength;
        }

        System.arraycopy(original, 0, newArray, 0, usedLength);
        return newArray;
    }

    public static int deepHashCode(Object[] arr) {
        // TODO make better
        return arr.length;
    }

    public static boolean equals(Object[] array1, Object[] array2) {
        if (array1 == array2) {
            return true;
        }

        if (array1 == null || array2 == null) {
            return false;
        }

        if (array1.length != array2.length) {
            return false;
        }

        for (int i = 0; i < array1.length; i ++) {
            Object elem1 = array1[i];
            Object elem2 = array2[i];

            if (elem1 == null) {
                if (elem2 != null) {
                    return false;
                }
            } else if (!elem1.equals(elem2)) {
                return false;
            }
        }

        return true;
    }

    public static boolean equals(int[] array1, int[] array2) {
        if (array1 == array2) {
            return true;
        }

        if (array1 == null || array2 == null) {
            return false;
        }

        if (array1.length != array2.length) {
            return false;
        }

        for (int i = 0; i < array1.length; i ++) {
            if (array1[i] != array2[i]) {
                return false;
            }
        }

        return true;
    }

    public static void fill(int[] arr, int fromIndex, int toIndex, int val) {
        if (fromIndex > toIndex) {
            throw new IllegalArgumentException();
        }

        for (int i = fromIndex; i < toIndex; i ++) {
            arr[i] = val;
        }
    }

    public static int hashCode(int[] arr) {
        // TODO make better
        int result = 0;
        for (int i = 0; i < arr.length; i ++) {
            result += i;
        }
        return result;
    }

    public static void sort(Object[] arr) {
        Arrays.sort(arr, 0, arr.length);
    }

    public static void sort(Object[] arr, int from, int to) {
        if (to - from < 2) {
            return;
        }

        // TODO faster sort
        boolean needsSort = true;
        while (needsSort) {
            needsSort = false;

            for (int i = from; i < to - 1; i ++) {
                Comparable firstElem = (Comparable) arr[i];
                Comparable secondElem = (Comparable) arr[i + 1];
                if (firstElem.compareTo(secondElem) > 0) {
                    needsSort = true;

                    Comparable temp = firstElem;
                    arr[i] = secondElem;
                    arr[i + 1] = temp;
                }
            }
        }
    }

    public static String toString(Object[] arr) {
        if (arr == null) {
            return "null";
        }

        StringBuilder result = new StringBuilder();
        result.append('[');
        for (int i = 0; i < arr.length; i ++) {
            result.append(String.valueOf(arr[i]));
            if (i != arr.length - 1) {
                result.append(", ");
            }
        }
        result.append(']');
        return result.toString();
    }
}

class ArrayBackedList<E> extends AbstractList<E> {
    private Object[] data;

    ArrayBackedList(Object[] data) {
        this.data = data;
    }

    public E get(int index) {
        return (E) this.data[index];
    }

    public E set(int index, E element) {
        E prev = (E) this.data[index];
        this.data[index] = element;
        return prev;
    }

    public int size() {
        return this.data.length;
    }

    public void clear() {
        // TODO remove this once `AbstractList.clear` is implemented
        throw new UnsupportedOperationException();
    }

    public Iterator<E> iterator() {
        // TODO remove this once `AbstractList.iterator` is implemented
        return new ArrayIterator<E>(this.data, this.data.length);
    }
}
