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
            System.out.println(A.class);
            System.out.println(new A());
        } catch(Throwable e) {
            System.out.println("Caught " + e.getClass());
        }
        try {
            System.out.println(A.class);
            System.out.println(new A());
        } catch(Throwable e) {
            System.out.println("Caught " + e.getClass());
        }
        try {
            System.out.println(A.class);
            System.out.println(Class.forName("A"));
        } catch(Throwable e) {
            System.out.println("Caught " + e.getClass());
        }
        try {
            System.out.println((new A[1]).getClass());
        } catch(Throwable e) {
            System.out.println("Caught " + e.getClass());
        }

        otherMethod();
    }

    public static void otherMethod() {
        System.out.println(A.class);
    }
}
