package java.awt;

public abstract class FontMetrics {
    protected Font font;

    protected FontMetrics(Font font) {
        this.font = font;
    }

    public Font getFont() {
        return this.font;
    }

    public int getAscent() {
        // TODO
        return this.font.getSize();
    }

    public int getDescent() {
        // TODO
        return this.font.getSize() / 5;
    }

    public int charWidth(char c) {
        // TODO
        return this.font.getSize() / 2;
    }
}
