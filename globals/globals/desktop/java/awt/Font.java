package java.awt;

public class Font {
    public static final int PLAIN = 0;
    public static final int BOLD = 1;
    public static final int ITALIC = 2;

    protected String name;
    protected int style;
    protected int size;

    public Font(String name, int style, int size) {
        this.name = name;
        this.style = style;
        this.size = size;
    }

    public Font(Font font) {
        this(font.name, font.style, font.size);
    }

    public String getName() {
        return this.name;
    }

    public int getSize() {
        return this.size;
    }
}
