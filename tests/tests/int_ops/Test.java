public class Test {
    public static void main(String[] args) {
        int v0 = 0;
        int v1 = 3;
        int v2 = 9;
        int v3 = 1;
        int v4 = 4;

        System.out.println(v1 + v2);
        System.out.println(v2 - v3);
        System.out.println(v3 - v1);
        System.out.println((v3 * v2) + v1);
        System.out.println(v4 / v2);
        try {
            System.out.println(v4 / v0);
        } catch(ArithmeticException e) {
            System.out.println("Division by 0 threw exception");
        }
    }
}
