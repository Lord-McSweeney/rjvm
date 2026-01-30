class A {
    static {
        System.out.println("<clinit> for class A");
    }
}

interface B {
    int staticF = initializer();
    static int initializer() {
        System.out.println("<clinit> for class B");
        return 12;
    }
}

interface C extends B {
    int staticF = initializer();
    static int initializer() {
        System.out.println("<clinit> for class C");
        return 13;
    }
}

class D extends A implements B  {
    static {
        System.out.println("<clinit> for class D");
    }
}

class E extends D implements C {
    static {
        System.out.println("<clinit> for class E");
    }
    
    public String toString() {
        return "[E object]";
    }
}

public class Test {
    public static void main(String[] args) {
        System.out.println(new E());
    }
}
