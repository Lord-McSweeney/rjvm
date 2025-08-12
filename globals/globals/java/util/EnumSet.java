package java.util;

import rjvm.internal.Todo;

public abstract class EnumSet<E extends Enum<E>> extends AbstractSet<E> {
    public static <E extends Enum<E>> EnumSet<E> allOf(Class<E> elementType) {
        Todo.warnNotImpl("java.util.EnumSet.allOf");

        return null;
    }
}
