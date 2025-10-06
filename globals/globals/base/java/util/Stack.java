package java.util;

public class Stack<E> extends Vector<E> {
    public E pop() {
        if (this.size() == 0) {
            throw new ArrayIndexOutOfBoundsException();
        }
        return this.remove(this.size() - 1);
    }

    public boolean empty() {
        return this.size() == 0;
    }
}
