package java.awt;

public class Color {
    public static final Color BLACK = new Color(0, 0, 0);
    public static final Color DARK_GRAY = new Color(64, 64, 64);
    public static final Color GRAY = new Color(128, 128, 128);
    public static final Color LIGHT_GRAY = new Color(192, 192, 192);
    public static final Color WHITE = new Color(255, 255, 255);

    public static final Color BLUE = new Color(0, 0, 255);
    public static final Color CYAN = new Color(0, 255, 255);
    public static final Color GREEN = new Color(0, 255, 0);
    public static final Color MAGENTA = new Color(255, 0, 255);
    public static final Color ORANGE = new Color(255, 200, 0);
    public static final Color PINK = new Color(255, 175, 175);
    public static final Color RED = new Color(255, 0, 0);
    public static final Color YELLOW = new Color(255, 255, 0);

    private int r;
    private int g;
    private int b;
    private int a;

    public Color(int r, int g, int b) {
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = 255;
    }

    public Color(int r, int g, int b, int a) {
        this.r = r;
        this.g = g;
        this.b = b;
        this.a = a;
    }

    public int getAlpha() {
        return this.a;
    }
}
