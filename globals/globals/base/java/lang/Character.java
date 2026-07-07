package java.lang;

import rjvm.internal.Todo;

public final class Character extends Number implements Comparable<Character> {
    public static Class<Character> TYPE = (Class<Character>) Class.getPrimitiveClass(Class.PRIM_CHAR);

    public static final int MIN_RADIX = 2;

    public static final int MAX_RADIX = 36;

    // `Character.getType` results
    public static final byte UNASSIGNED = 0;
    public static final byte UPPERCASE_LETTER = 1;
    public static final byte LOWERCASE_LETTER = 2;
    public static final byte TITLECASE_LETTER = 3;
    public static final byte MODIFIER_LETTER = 4;
    public static final byte OTHER_LETTER = 5;
    public static final byte NON_SPACING_MARK = 6;
    public static final byte ENCLOSING_MARK = 7;
    public static final byte COMBINING_SPACING_MARK = 8;
    public static final byte DECIMAL_DIGIT_NUMBER = 9;
    public static final byte LETTER_NUMBER = 10;
    public static final byte OTHER_NUMBER = 11;
    public static final byte SPACE_SEPARATOR = 12;
    public static final byte LINE_SEPARATOR = 13;
    public static final byte PARAGRAPH_SEPARATOR = 14;
    public static final byte CONTROL = 15;
    public static final byte FORMAT = 16;
    // 17?
    public static final byte PRIVATE_USE = 18;
    public static final byte SURROGATE = 19;
    public static final byte DASH_PUNCTUATION = 20;
    public static final byte START_PUNCTUATION = 21;
    public static final byte END_PUNCTUATION = 22;
    public static final byte CONNECTOR_PUNCTUATION = 23;
    public static final byte OTHER_PUNCTUATION = 24;
    public static final byte MATH_SYMBOL = 25;
    public static final byte CURRENCY_SYMBOL = 26;
    public static final byte MODIFIER_SYMBOL = 27;
    public static final byte OTHER_SYMBOL = 28;
    public static final byte INITIAL_QUOTE_PUNCTUATION = 29;
    public static final byte FINAL_QUOTE_PUNCTUATION = 30;

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

    public char charValue() {
        return this.value;
    }

    public int intValue() {
        return (int) this.value;
    }

    public float floatValue() {
        return (float) this.value;
    }

    public long longValue() {
        return (long) this.value;
    }

    public double doubleValue() {
        return (double) this.value;
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

    public static int codePointAt(char[] chars, int index) {
        // TODO implement
        return chars[index];
    }

    public static int codePointAt(CharSequence chars, int index) {
        // TODO implement
        return chars.charAt(index);
    }

    public static int codePointBefore(char[] chars, int index) {
        // TODO implement
        return chars[index - 1];
    }

    public static int codePointBefore(CharSequence chars, int index) {
        // TODO implement
        return chars.charAt(index - 1);
    }

    public static int digit(char c, int radix) {
        Todo.warnNotImpl("java.lang.Character.digit");

        return 0;
    }

    public static int getType(char c) {
        return CharacterTables.typeOf((int) c);
    }

    public static int getType(int c) {
        return CharacterTables.typeOf(c);
    }

    public static boolean isDefined(char c) {
        return Character.getType(c) != Character.UNASSIGNED;
    }

    public static boolean isJavaIdentifierStart(char c) {
        // TODO more...
        return Character.isLetter(c) || (c == '_') || (c == '$');
    }

    public static boolean isJavaIdentifierPart(char c) {
        // TODO more...
        return Character.isJavaIdentifierStart(c) || (c >= '0' && c <= '9');
    }

    public static boolean isJavaLetter(char c) {
        // TODO more...
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
    }

    public static boolean isJavaLetterOrDigit(char c) {
        // TODO more...
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9');
    }

    public static boolean isLetter(char c) {
        int type = Character.getType(c);

        return type == Character.UPPERCASE_LETTER || type == Character.LOWERCASE_LETTER || type == Character.TITLECASE_LETTER || type == Character.MODIFIER_LETTER || type == Character.OTHER_LETTER;
    }

    public static boolean isLowerCase(char c) {
        return Character.isLowerCase((int) c);
    }

    public static boolean isLowerCase(int c) {
        int type = Character.getType(c);

        // TODO more?
        return type == Character.LOWERCASE_LETTER;
    }

    public static boolean isUpperCase(char c) {
        return Character.isUpperCase((int) c);
    }

    public static boolean isUpperCase(int c) {
        int type = Character.getType(c);

        // TODO more?
        return type == Character.UPPERCASE_LETTER;
    }

    public static boolean isDigit(char c) {
        return Character.isDigit((int) c);
    }

    public static boolean isDigit(int c) {
        int type = Character.getType(c);

        return type == Character.DECIMAL_DIGIT_NUMBER;
    }

    public static boolean isSpace(char c) {
        // These are the only characters that isSpace returns true for
        return c == ' ' || c == '\f' || c == '\t' || c == '\r' || c == '\n';
    }

    public static boolean isWhitespace(char c) {
        return Character.isWhitespace((int) c);
    }

    public static boolean isWhitespace(int c) {
        // TODO more characters
        return c == ' ' || c == '\f' || c == '\t' || c == '\r' || c == '\n';
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
