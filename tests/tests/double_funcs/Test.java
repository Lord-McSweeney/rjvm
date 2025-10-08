public class Test {
    public static void main(String[] args) {
        double inf = 1.0 / 0.0;
        double negInf = 1.0 / 0.0;
        double nan = 0.0 / 0.0;
        double negNan = -0.0 / 0.0;
        double normalDouble = 3.125;
        double largeDouble = 3728347837.125;
        double negDouble = -837.125;
        double zeroDouble = 0.0;
        double negZeroDouble = -0.0;
        double largeNeg = -723874.875;
        double largeImprec = 327487482.1236667;
        double largeNegImprec = -723874.367899;
        double[] doubles = new double[]{inf, negInf, nan, negNan, normalDouble, largeDouble, negDouble, zeroDouble, negZeroDouble, largeNeg, largeImprec, largeNegImprec};
        for (int i = 0; i < doubles.length; i ++) {
            double dob = doubles[i];
            Double obj = Double.valueOf(dob);

            System.out.println(obj.isInfinite());
            System.out.println(Double.isNaN(dob));
            System.out.println(obj.isNaN());
            System.out.println(Double.doubleToLongBits(dob));
            System.out.println(Double.doubleToRawLongBits(dob));
            if (dob < 100000000) {
                System.out.println(Double.toString(dob));
            }
        }
        String[] doubleStrs = new String[]{"1.0", "0.0", "-0.0", "-.0", "-0", "0", "1", "673", "68723.273", "127.125", "-78.25", "Infinity", "-Infinity", "NaN", "-NaN", "   7384", "  89.3   \t", "- 3", "-", "+"};
        for (int i = 0; i < doubleStrs.length; i ++) {
            try {
                System.out.println(Double.parseDouble(doubleStrs[i]));
            } catch(NumberFormatException e) {
                System.out.println("failed to parse");
            }
        }
    }
}
