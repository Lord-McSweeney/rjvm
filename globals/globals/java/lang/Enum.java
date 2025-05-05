package java.lang;

public abstract class Enum<E extends Enum<E>> {
    private int ordinal;

    protected Enum(String name, int ordinal) {
        super();
        this.ordinal = ordinal;
    }

    public final int ordinal() {
        return this.ordinal;
    }

    public static <T extends Enum<T>> T valueOf(Class<T> enumType, String name) {
        // TODO implement
        return null;
    }
}
