public class Test {
    public static void main(String[] args) {
        double inf = 1.0 / 0.0;
        double negInf = 1.0 / 0.0;
        double nan = 0.0 / 0.0;
        double negNan = -0.0 / 0.0;
        double normalDouble = 3.125;
        double largeDouble = 3728347837.125;
        double negDouble = -837.125;
        double[] doubles = new double[]{inf, negInf, nan, negNan, normalDouble, largeDouble, negDouble};
        for (int i = 0; i < doubles.length; i ++) {
            double dob = doubles[i];
            Double obj = Double.valueOf(dob);

            System.out.println(obj.isInfinite());
            System.out.println(Double.isNaN(dob));
            System.out.println(obj.isNaN());
            System.out.println(Double.doubleToLongBits(dob));
            System.out.println(Double.doubleToRawLongBits(dob));
        }
    }
}
