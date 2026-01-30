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

    void impl();
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

    public void impl() {
        System.out.println("impl called (D)");
    }
}

class E extends D implements C {
    static {
        System.out.println("<clinit> for class E");
    }

    int field = 99;

    public void impl() {
        System.out.println("impl called (E)");
    }

    public static void staticMethod() {
        System.out.println("static method called (E)");
    }

    public String toString() {
        return "[E object]";
    }
}

public class Test {
    public static void main(String[] args) {
        try {
            E conv = null;
            conv.toString();
        } catch(Exception e) {
            System.out.println("exception caught (1)");
        }
        try {
            E conv = null;
            System.out.println(conv.field);
        } catch(Exception e) {
            System.out.println("exception caught (2)");
        }
        try {
            E conv = null;
            conv.field = 9;
        } catch(Exception e) {
            System.out.println("exception caught (3)");
        }
        System.out.println("Calling static");
        E.staticMethod();
        E e = new E();
        System.out.println(e instanceof C);
        System.out.println((C) e);
        C[] array = new C[1];
        System.out.println(array.length);
    }
}
