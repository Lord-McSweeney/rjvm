package java.awt.geom;

public abstract class Point2D implements Cloneable {
    protected Point2D() { }

    public abstract double getX();

    public abstract double getY();

    public abstract void setLocation(double x, double y);

    public boolean equals(Object o) {
        if (!(o instanceof Point2D)) return false;

        Point2D other = (Point2D) o;

        return this.getX() == other.getX() && this.getY() == other.getY();
    }
}
