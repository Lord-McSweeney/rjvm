package java.math;

public enum RoundingMode {
    HALF_EVEN(BigDecimal.ROUND_HALF_EVEN);

    private int modeInt;

    private RoundingMode(int modeInt) {
        this.modeInt = modeInt;
    }
}
