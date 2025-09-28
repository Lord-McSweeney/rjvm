package java.lang;

public final class Character extends Number implements Comparable<Character> {
    public static Class<Character> TYPE = (Class<Character>) Class.getPrimitiveClass(Class.PRIM_CHAR);

    public static final int MIN_RADIX = 2;

    public static final int MAX_RADIX = 36;

    private char value;

    public Character(char value) {
        this.value = value;
    }

    public boolean equals(Object obj) {
        if (obj instanceof Character) {
            Character other = (Character) obj;
            return this.value == other.value;
        } else {
            return false;
        }
    }

    public int compareTo(Character other) {
        if (this.value < other.value) {
            return -1;
        } else if (this.value == other.value) {
            return 0;
        } else {
            return 1;
        }
    }

    public static int digit(char c, int radix) {
        // TODO implement
        return 0;
    }

    public static int getType(char ch) {
        // TODO implement
        return 0;
    }

    public static boolean isDefined(char ch) {
        // TODO implement
        return false;
    }

    public static boolean isJavaIdentifierPart(char ch) {
        // TODO implement
        return false;
    }
}
