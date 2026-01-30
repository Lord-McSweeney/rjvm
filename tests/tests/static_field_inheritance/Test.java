interface D {
    public static int[] staticD = new int[4];
}

interface C extends D {
    public static int[] staticC = new int[3];
}

class B {
    public static int[] staticB = new int[2];
}

class A extends B implements C {
    public static int[] staticA = new int[1];
}

public class Test {
    public static void main(String[] args) {
        System.out.println(A.staticA.length);
        System.out.println(A.staticB.length);
        System.out.println(B.staticB.length);
        System.out.println(A.staticC.length);
        System.out.println(C.staticC.length);
        System.out.println(A.staticD.length);
        System.out.println(C.staticD.length);
        System.out.println(D.staticD.length);

        System.out.println(A.staticB == B.staticB);
        System.out.println(A.staticC == C.staticC);
        System.out.println(C.staticD == D.staticD);
        System.out.println(A.staticD == C.staticD);

        A.staticB = new int[5];
        A.staticA = new int[6];

        System.out.println(A.staticA.length);
        System.out.println(A.staticB.length);
        System.out.println(B.staticB.length);
        System.out.println(A.staticC.length);
        System.out.println(C.staticC.length);
        System.out.println(A.staticD.length);
        System.out.println(C.staticD.length);
        System.out.println(D.staticD.length);
    }
}
