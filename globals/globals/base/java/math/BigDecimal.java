package java.math;

import rjvm.internal.Todo;

public class BigDecimal extends Number implements Comparable<BigDecimal> {
    public static int ROUND_HALF_EVEN = 6;

    public BigDecimal(String value) {
        // TODO implement
    }

    public int intValue() {
        return 0;
    }

    public float floatValue() {
        return 0.0f;
    }

    public long longValue() {
        return 0L;
    }

    public double doubleValue() {
        return 0.0;
    }

    public int compareTo(BigDecimal other) {
        Todo.warnNotImpl("java.math.BigDecimal.compareTo");

        return 0;
    }
}
