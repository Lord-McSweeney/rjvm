public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(Test.class.isInterface());
        System.out.println(Test.class.getName());
        System.out.println(Test.class.getClass().getName());
        System.out.println(Comparable.class.getName());
        System.out.println(Comparable.class.getName());
        System.out.println(Comparable.class.isInterface());
        System.out.println(System.class.isInterface());
        System.out.println(Test.class.getResourceAsStream("Test.class").read());
        System.out.println(Comparable.class.isPrimitive());
        System.out.println(System.class.isPrimitive());
        System.out.println(Comparable.class);
        System.out.println(System.class);
    }
}
