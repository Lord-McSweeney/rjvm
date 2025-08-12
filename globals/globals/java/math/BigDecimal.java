package java.math;

import rjvm.internal.Todo;

public class BigDecimal extends Number implements Comparable<BigDecimal> {
    public static int ROUND_HALF_EVEN = 6;

    public BigDecimal(String value) {
        // TODO implement
    }

    public int compareTo(BigDecimal other) {
        Todo.warnNotImpl("java.math.BigDecimal.compareTo");

        return 0;
    }
}
