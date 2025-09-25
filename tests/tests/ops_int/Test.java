public class Test {
    public static void main(String[] args) {
        int v0 = 0;
        int v1 = 3;
        int v2 = 9;
        int v3 = 1;
        int v4 = 4;
        int v5 = 9;
        int v6 = -3;
        int v7 = 3;

        System.out.println(v1 + v2);
        System.out.println(v2 - v3);
        System.out.println(v3 - v1);
        System.out.println((v3 * v2) + v1);
        System.out.println(v4 / v2);
        System.out.println(-v2);
        System.out.println(v2 % v4);
        System.out.println(v4 | v2);
        System.out.println(v2 | v1);
        System.out.println(v1 & v4);
        System.out.println(v2 & v4);
        System.out.println(~v3);
        System.out.println(~v2);
        System.out.println(v3 ^ v1);
        System.out.println(v2 ^ v4);
        System.out.println(v6 >>> v3);
        System.out.println(v6 << v7);
        System.out.println(v6 >> v7);
        try {
            System.out.println(v4 / v0);
        } catch(ArithmeticException e) {
            System.out.println("Division by 0 threw exception");
        }
        try {
            System.out.println(v3 % v0);
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
}
