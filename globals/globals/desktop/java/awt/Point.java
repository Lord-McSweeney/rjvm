package java.awt;

import java.awt.geom.Point2D;

public class Point extends Point2D {
    public int x;
    public int y;

    public Point() {
        this(0, 0);
    }

    public Point(Point other) {
        this(other.x, other.y);
    }

    public Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    // Methods

    public double getX() {
        return this.x;
    }

    public double getY() {
        return this.y;
    }

    public void setLocation(double x, double y) {
        this.x = (int) Math.floor(x + 0.5);
        this.y = (int) Math.floor(y + 0.5);
    }

    public void move(int x, int y) {
        this.x = x;
        this.y = y;
    }
}
