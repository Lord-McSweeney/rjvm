package java.util.logging;

public class Level {
    public static final Level OFF = new Level("OFF", Integer.MAX_VALUE, null);
    public static final Level SEVERE = new Level("SEVERE", 1000, null);
    public static final Level WARNING = new Level("WARNING", 900, null);
    public static final Level INFO = new Level("INFO", 800, null);
    public static final Level CONFIG = new Level("CONFIG", 700, null);
    public static final Level FINE = new Level("FINE", 500, null);
    public static final Level FINER = new Level("FINER", 400, null);
    public static final Level FINEST = new Level("FINEST", 300, null);
    public static final Level ALL = new Level("ALL", Integer.MIN_VALUE, null);

    private final String name;

    private final int value;

    protected Level(String name, int value) {
        this(name, value, null);
    }

    protected Level(String name, int value, String resourceBundleName) {
        if (name == null) {
            throw new NullPointerException();
        }

        this.name = name;
        this.value = value;
    }
}
