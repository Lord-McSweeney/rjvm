package java.awt;

public abstract class Graphics {
    public abstract void drawLine(int x1, int y1, int x2, int y2);

    public abstract void drawString(String string, int x, int y);

    public abstract void fillRect(int x, int y, int width, int height);

    public abstract void fillPolygon(int xPoints[], int yPoints[], int nPoints);

    public abstract void setColor(Color color);

    public abstract void translate(int x, int y);
}
