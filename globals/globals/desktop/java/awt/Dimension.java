package java.awt;

import java.awt.geom.Dimension2D;

public class Dimension extends Dimension2D {
    public int width;
    public int height;

    public Dimension() {
        this(0, 0);
    }

    public Dimension(Dimension other) {
        this(other.width, other.height);
    }

    public Dimension(int width, int height) {
        this.width = width;
        this.height = height;
    }

    public double getWidth() {
        return this.width;
    }

    public double getHeight() {
        return this.height;
    }

    public void setSize(double width, double height) {
        this.width = (int) Math.ceil(width);
        this.height = (int) Math.ceil(height);
    }
}
