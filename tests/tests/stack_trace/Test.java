public class Test {
    public static void main(String[] args) {
        try {
            a();
        } catch(Exception e) {
            printThrowable(e);
            e.fillInStackTrace();
            printThrowable(e);
        }
    }

    public static void a() {
        printThrowable(new Throwable());
        b();
    }

    public static void b() {
        printThrowable(new LinkageError());
        c();
    }

    public static void c() {
        throw new RuntimeException();
    }

    public static void printThrowable(Throwable t) {
        System.out.println(t.getClass());
        for (StackTraceElement element : t.getStackTrace()) {
            System.out.println("    " + element.getClassName() + "-" + element.getMethodName());
        }
        System.out.println();
    }
}
