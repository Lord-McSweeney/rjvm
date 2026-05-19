package java.awt;

import java.awt.image.ImageObserver;

public abstract class Graphics {
    public abstract boolean drawImage(Image img, int x, int y, ImageObserver observer);

    public abstract void drawLine(int x1, int y1, int x2, int y2);

    public abstract void drawString(String string, int x, int y);

    public abstract void fillPolygon(int xPoints[], int yPoints[], int nPoints);

    public abstract void fillRect(int x, int y, int width, int height);

    public abstract Font getFont();

    public abstract FontMetrics getFontMetrics(Font font);

    public abstract void setColor(Color color);

    public abstract void setFont(Font font);

    public abstract void translate(int x, int y);

    public void drawRect(int x, int y, int width, int height) {
        if (width < 0 || height < 0) {
            return;
        }

        if (height == 0) {
            this.drawLine(x, y, x + width, y);
        } else if (width == 0) {
            this.drawLine(x, y, x, y + height);
        } else {
            this.drawLine(x, y, (x + width) - 1, y);
            this.drawLine(x + width, y + height, x + 1, y + height);
            this.drawLine(x + width, y, x + width, (y + height) - 1);
            this.drawLine(x, y + height, x, y + 1);
        }
    }

    public FontMetrics getFontMetrics() {
        return this.getFontMetrics(this.getFont());
    }
}
