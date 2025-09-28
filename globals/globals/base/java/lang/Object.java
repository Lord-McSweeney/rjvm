package java.lang;

public class Object {
    public Object() { }

    public final native Class<?> getClass();

    // NOTE method ordering is important
    public String toString() {
        String hexHashCode = Integer.toHexString(this.hashCode());
        return this.getClass().getName() + "@" + hexHashCode;
    }

    public native int hashCode();

    public boolean equals(Object other) {
        return this == other;
    }

    protected native Object clone() throws CloneNotSupportedException;
}
