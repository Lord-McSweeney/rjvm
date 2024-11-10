package java.lang;

public abstract class Enum<E extends Enum<E>> {
    protected Enum(String name, int ordinal) {
        super();
    }

    public static <T extends Enum<T>> T valueOf(Class<T> enumType, String name) {
        // TODO implement
        return null;
    }
}
