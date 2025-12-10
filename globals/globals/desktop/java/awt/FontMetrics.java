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
        return this.font.getSize();
    }
}
