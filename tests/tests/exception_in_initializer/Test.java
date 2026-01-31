class A {
    static {
        if (true) {
            throw new RuntimeException();
        }
    }
}

public class Test {
    public static void main(String[] args) {
        try {
            new A();
        } catch(Throwable e) {
            System.out.println(e.getClass().getName());
            System.out.println(e.getCause().getClass().getName());
        }
    }
}
