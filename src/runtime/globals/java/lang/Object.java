package java.lang;

public class Object {
    public Object() { }

    public final native Class<?> getClass();

    public String toString() {
        // TODO implement
        return this.getClass().getName() + "@0";
    }

    protected Object clone() throws CloneNotSupportedException {
        throw new CloneNotSupportedException();
    }
}
