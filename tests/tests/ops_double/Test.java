public class Test {
    public static void main(String[] args) {
        double v0 = 0;
        double v1 = 323.367;
        double v2 = 917.999;
        double v3 = (0.0 / 0.0); // NaN
        double v4 = 488.982;
        double v5 = 917.999;

        printDouble(v1 + v2);
        printDouble(v2 - v3);
        printDouble(v3 - v1);
        printDouble((v3 * v2) + v1);
        printDouble(v4 / v2);
        printDouble(-v2);
        printDouble(v2 % v4);
        try {
            printDouble(v4 / v0);
        } catch(ArithmeticException e) {
            System.out.println("Division by 0 threw exception");
        }
        try {
            printDouble(v3 % v0);
        } catch(ArithmeticException e) {
            System.out.println("Modulo by 0 threw exception");
        }

        System.out.println(v0 > v1);
        System.out.println(v0 == v1);
        System.out.println(v0 < v1);
        System.out.println(v2 > v3);
        System.out.println(v2 == v5);
        System.out.println(v2 < v3);
        if (v2 < v3) {
            System.out.println("v2 < v3");
        }
        if (v4 > v5) {
            System.out.println("v4 > v5");
        }
        if (v2 > v3) {
            System.out.println("v2 > v3");
        }
        if (v0 > v1) {
            System.out.println("v0 > v1");
        }
        if (v2 == v5) {
            System.out.println("v2 == v5");
        }
        if (v0 == v1) {
            System.out.println("v0 == v1");
        }
    }
    
    static void printDouble(double d) {
        System.out.println((int) (d * 1000));
    }
}
