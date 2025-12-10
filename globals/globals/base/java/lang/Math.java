package java.lang;

public class Math {
    public static double abs(double v) {
        // FIXME are we handling negative zero correctly?
        if (v < 0) {
            return -v;
        } else {
            return v;
        }
    }

    public static native double atan2(double y, double x);

    public static native double floor(double v);

    public static native double log(double v);

    public static int max(int a, int b) {
        if (a > b) {
            return a;
        } else {
            return b;
        }
    }

    public static int min(int a, int b) {
        if (a < b) {
            return a;
        } else {
            return b;
        }
    }

    public static native double pow(double b, double e);

    public static double signum(double v) {
        if (v == v) {
            // not-NaN
            if (v == 0) {
                // `return v` handles -0 correctly
                return v;
            } else if (v < 0) {
                return -1.0;
            } else {
                return 1.0;
            }
        } else {
            // NaN
            return v;
        }
    }

    public static native double sqrt(double v);
}
