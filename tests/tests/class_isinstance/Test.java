public class Test {
    public static void main(String[] args) {
        System.out.println(Throwable.class.isInstance(null));
        System.out.println(Throwable.class.isInstance(new Throwable()));
        System.out.println(Throwable.class.isInstance(new Exception()));
        System.out.println(Throwable.class.isInstance(new ArithmeticException()));
        System.out.println(Exception.class.isInstance(new Throwable()));
        System.out.println(Exception.class.isInstance(new Exception()));
        System.out.println(Comparable.class.isInstance(Integer.valueOf(0)));
        System.out.println(Comparable.class.isInstance(null));
        System.out.println(Comparable.class.isInstance(new Test()));
        System.out.println(Class.class.isInstance(Test.class));
        System.out.println(String.class.isInstance(""));
        System.out.println(String.class.isInstance(new Object()));
    }
}
