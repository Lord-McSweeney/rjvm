interface A {
    void method();
}

abstract class B implements A { }

class C extends B {
    public void method() {
        System.out.println("Method called");
    }
}

public class Test {
    public static void main(String[] args) {
        B b = new C();
        b.method();
    }
}
