class A {
    static {
        System.out.println("A init started");
        new B();
        System.out.println("A init ended");
    }
}

class B extends A {
    static {
        System.out.println("B init started");
        new A();
        System.out.println("B init ended");
    }
}

class C extends B { }

public class Test {
    public static void main(String[] args) throws Exception {
        new B();
        new C();
    }
}
