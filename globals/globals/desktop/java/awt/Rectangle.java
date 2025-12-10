package java.awt;

import java.awt.geom.Rectangle2D;

public class Rectangle extends Rectangle2D implements Shape {
    public int x;
    public int y;
    public int width;
    public int height;

    public Rectangle() {
        this(0, 0, 0, 0);
    }

    public Rectangle(int x, int y, int width, int height) {
        this.x = x;
        this.y = y;
        this.width = width;
        this.height = height;
    }

    public boolean inside(int testX, int testY) {
        if (this.width < 0 || this.height < 0) {
            return false;
        }

        if (testX < this.x || testY < this.y) {
            return false;
        }

        return (testX < this.x + this.width) && (testY < this.y + this.height);
    }
}
