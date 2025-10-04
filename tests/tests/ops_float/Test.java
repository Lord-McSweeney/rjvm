public class Test {
    public static void main(String[] args) {
        float v0 = 0F;
        float v1 = 323.367F;
        float v2 = 917.999F;
        float v3 = (0.0F / 0.0F); // NaN
        float v4 = 488.982F;
        float v5 = 917.999F;

        printFloat(v1 + v2);
        printFloat(v2 - v3);
        printFloat(v3 - v1);
        printFloat((v3 * v2) + v1);
        printFloat(v4 / v2);
        printFloat(-v2);
        printFloat(v2 % v4);
        try {
            printFloat(v4 / v0);
        } catch(ArithmeticException e) {
            System.out.println("Division by 0 threw exception");
        }
        try {
            printFloat(v3 % v0);
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
    
    static void printFloat(float d) {
        System.out.println((int) (d * 1000));
    }
}
