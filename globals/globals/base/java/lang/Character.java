package java.lang;

import rjvm.internal.Todo;

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

    public static int charCount(int codePoint) {
        if (codePoint >= 0x10000) {
            return 2;
        } else {
            return 1;
        }
    }

    public static char[] toChars(int codePoint) {
        // TODO implement
        return new char[]{(char) codePoint};
    }

    public static int digit(char c, int radix) {
        Todo.warnNotImpl("java.lang.Character.digit");

        return 0;
    }

    public static int getType(char c) {
        Todo.warnNotImpl("java.lang.Character.getType");

        return 0;
    }

    public static boolean isDefined(char c) {
        Todo.warnNotImpl("java.lang.Character.isDefined");

        return false;
    }

    public static boolean isJavaIdentifierStart(char c) {
        // TODO more...
        return Character.isLetter(c) || (c == '_') || (c == '$');
    }

    public static boolean isJavaIdentifierPart(char c) {
        // TODO more...
        return Character.isJavaIdentifierStart(c) || (c >= '0' && c <= '9');
    }

    public static boolean isLetter(char c) {
        // TODO more...
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    }

    public static boolean isLowerCase(char c) {
        return Character.isLowerCase((int) c);
    }

    public static boolean isLowerCase(int c) {
        // TODO more...
        return c >= 'a' && c <= 'z';
    }

    public static boolean isUpperCase(char c) {
        return Character.isUpperCase((int) c);
    }

    public static boolean isUpperCase(int c) {
        // TODO more...
        return c >= 'A' && c <= 'Z';
    }

    public static boolean isDigit(char c) {
        // TODO more...
        return c >= '0' && c <= '9';
    }

    public static boolean isDigit(int c) {
        // TODO more...
        return c >= '0' && c <= '9';
    }

    public static boolean isWhitespace(int c) {
        // TODO more characters
        return c == ' ' || c == '\t' || c == '\r' || c == '\n';
    }

    public static char toLowerCase(char c) {
        return (char) Character.toLowerCase((int) c);
    }

    public static int toLowerCase(int codePoint) {
        if (Character.isUpperCase(codePoint)) {
            // TODO more
            return codePoint + ('a' - 'A');
        } else {
            return codePoint;
        }
    }

    public static char toUpperCase(char c) {
        return (char) Character.toUpperCase((int) c);
    }

    public static int toUpperCase(int codePoint) {
        if (Character.isLowerCase(codePoint)) {
            // TODO more
            return codePoint - ('a' - 'A');
        } else {
            return codePoint;
        }
    }
}
