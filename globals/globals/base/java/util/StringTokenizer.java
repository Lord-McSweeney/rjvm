package java.util;

public class StringTokenizer {
    private String[] tokens;
    private int nextToken;
    private String delimeter;

    public StringTokenizer(String string) {
        // TODO default delimeter is all whitespace chars
        this(string, " ");
    }

    public StringTokenizer(String string, String delimeter) {
        if (string == null) {
            throw new NullPointerException();
        }

        this.tokens = string.split(delimeter);
        this.nextToken = 0;
        this.delimeter = delimeter;
    }

    public int countTokens() {
        return this.tokens.length;
    }

    public String nextToken() {
        if (this.nextToken == this.tokens.length) {
            throw new NoSuchElementException();
        }

        return this.tokens[this.nextToken ++];
    }
}
