package java.lang;

public class Object {
    public Object() { }

    public final native Class<?> getClass();

    public String toString() {
        // TODO implement
        return this.getClass().getName() + "@0";
    }

    public int hashCode() {
        // TODO implement
        return 0;
    }

    public boolean equals(Object other) {
        return this == other;
    }

    protected Object clone() throws CloneNotSupportedException {
        throw new CloneNotSupportedException();
    }
}
