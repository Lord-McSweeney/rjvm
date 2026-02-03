package java.lang;

import rjvm.internal.Todo;

public abstract class Enum<E extends Enum<E>> {
    private String name;
    private int ordinal;

    protected Enum(String name, int ordinal) {
        super();
        this.name = name;
        this.ordinal = ordinal;
    }

    public final String name() {
        return this.name;
    }

    public final int ordinal() {
        return this.ordinal;
    }

    public String toString() {
        return this.name;
    }

    public final boolean equals(Object o) {
        return this == o;
    }

    public static <T extends Enum<T>> T valueOf(Class<T> enumType, String name) {
        Todo.warnNotImpl("java.lang.Enum.valueOf");

        return null;
    }
}
