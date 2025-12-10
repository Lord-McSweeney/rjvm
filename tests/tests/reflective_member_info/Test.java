public class Test {
    public static void main(String[] args) throws Throwable {
        System.out.println(Test.class.getMethod("main", new Class[]{String[].class}).getDeclaringClass());
        System.out.println(Test.class.getMethod("main", new Class[]{String[].class}).getName());
        System.out.println(Test.class.getConstructors()[0].getDeclaringClass());
        System.out.println(Test.class.getConstructors()[0].getName());
    }
}
