public class Test {
    public static void main(String[] args) {
        System.out.println(Throwable.class.isAssignableFrom(Throwable.class));
        System.out.println(Throwable.class.isAssignableFrom(Exception.class));
        System.out.println(Throwable.class.isAssignableFrom(ArithmeticException.class));
        System.out.println(Exception.class.isAssignableFrom(Throwable.class));
        System.out.println(Exception.class.isAssignableFrom(Exception.class));
        System.out.println(Comparable.class.isAssignableFrom(Integer.class));
        System.out.println(Comparable.class.isAssignableFrom(Test.class));
        System.out.println(Class.class.isAssignableFrom(Class.class));
        System.out.println(String.class.isAssignableFrom(String.class));
        System.out.println(String.class.isAssignableFrom(Object.class));
        System.out.println(Object[].class.isAssignableFrom(Object.class));
        System.out.println(Object[].class.isAssignableFrom(Object[].class));
        System.out.println(Object[].class.isAssignableFrom(Integer[].class));
        System.out.println(Integer[].class.isAssignableFrom(Integer[].class));
        System.out.println(Integer[].class.isAssignableFrom(Object.class));

        System.out.println(Throwable.class.isAssignableFrom(Throwable.class));
        System.out.println(Exception.class.isAssignableFrom(Throwable.class));
        System.out.println(ArithmeticException.class.isAssignableFrom(Throwable.class));
        System.out.println(Throwable.class.isAssignableFrom(Exception.class));
        System.out.println(Exception.class.isAssignableFrom(Exception.class));
        System.out.println(Integer.class.isAssignableFrom(Comparable.class));
        System.out.println(Test.class.isAssignableFrom(Comparable.class));
        System.out.println(Class.class.isAssignableFrom(Class.class));
        System.out.println(String.class.isAssignableFrom(String.class));
        System.out.println(Object.class.isAssignableFrom(String.class));
        System.out.println(Object.class.isAssignableFrom(Object[].class));
        System.out.println(Object[].class.isAssignableFrom(Object[].class));
        System.out.println(Integer[].class.isAssignableFrom(Object[].class));
        System.out.println(Integer[].class.isAssignableFrom(Integer[].class));
        System.out.println(Object.class.isAssignableFrom(Integer[].class));
    }
}
