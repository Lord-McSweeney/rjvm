package java.util;

public class LinkedList<E> extends AbstractSequentialList<E> implements Cloneable, List<E>, Queue<E> {
    class Node<E> {
        private E data;
        private Node<E> prev;
        private Node<E> next;
        
        Node(E data) {
            this.data = data;
        }

        public E getData() {
            return this.data;
        }

        public void setData(E element) {
            this.data = element;
        }

        Node<E> next() {
            return this.next;
        }

        Node<E> prev() {
            return this.prev;
        }

        void setNext(Node<E> node) {
            this.next = node;
        }

        void setPrev(Node<E> node) {
            this.prev = node;
        }
    }

    private Node<E> head;
    private Node<E> tail;
    private int size;

    public LinkedList() {
        this.head = new Node<E>(null);
        this.tail = new Node<E>(null);
        this.head.setNext(this.tail);
        this.tail.setPrev(this.head);
    }

    private Node<E> getNode(int index) {
        if (index < 0 || index >= this.size) {
            throw new IndexOutOfBoundsException();
        }

        if (index > this.size / 2) {
            Node<E> current = this.tail;
            for (int i = this.size - 1; i >= index; i --) {
                current = current.prev();
            }
            return current;
        } else {
            Node<E> current = this.head;
            for (int i = 0; i <= index; i ++) {
                current = current.next();
            }
            return current;
        }
    }

    public boolean add(E element) {
        Node<E> node = new Node<E>(element);
        this.tail.prev().setNext(node);
        node.setPrev(this.tail.prev());
        node.setNext(this.tail);
        this.tail.setPrev(node);
        this.size += 1;
        return true;
    }

    public void add(int index, E element) {
        if (index == this.size) {
            this.add(element);
        } else {
            Node<E> newNode = new Node<E>(element);
            Node<E> oldNode = this.getNode(index);
            oldNode.prev().setNext(newNode);
            newNode.setPrev(oldNode.prev());
            newNode.setNext(oldNode);
            oldNode.setPrev(newNode);
            this.size ++;
        }
    }

    public E get(int index) {
        return this.getNode(index).getData();
    }

    public E set(int index, E element) {
        Node<E> node = this.getNode(index);
        E oldData = node.getData();
        node.setData(element);
        return oldData;
    }

    public E remove(int index) {
        Node<E> oldNode = this.getNode(index);
        oldNode.prev().setNext(oldNode.next());
        oldNode.next().setPrev(oldNode.prev());
        this.size -= 1;
        return oldNode.getData();
    }

    public E removeFirst() {
        if (this.size == 0) {
            throw new NoSuchElementException();
        }

        Node<E> oldNode = this.head.next();
        this.head.setNext(oldNode.next());
        oldNode.next().setPrev(this.head);
        this.size -= 1;
        return oldNode.getData();
    }

    public int size() {
        return this.size;
    }

    public void clear() {
        this.head.setNext(this.tail);
        this.tail.setPrev(this.head);
    }

    // `Queue` functions
    public E poll() {
        if (this.size == 0) {
            return null;
        } else {
            return this.remove(0);
        }
    }

    public E element() {
        if (this.size == 0) {
            throw new NoSuchElementException();
        } else {
            return this.get(0);
        }
    }

    public E peek() {
        if (this.size == 0) {
            return null;
        } else {
            return this.get(0);
        }
    }
}
