public class Test {
    public static void main(String[] args) {
        System.out.println(Test.class.getResourceAsStream("Test.class") == null);
        System.out.println(Test.class.getResourceAsStream("java/lang/Integer.class") == null);
        System.out.println(Integer.class.getResourceAsStream("Test.class") == null);
        System.out.println(Integer.class.getResourceAsStream("Integer.class") == null);
        System.out.println(Test.class.getResourceAsStream("abcd") == null);
        System.out.println(Test.class.getResourceAsStream("Test.class") == null);
        System.out.println(Test.class.getResourceAsStream("java/lang/Integer.class") == null);
        System.out.println(Integer.class.getResourceAsStream("Test.class") == null);
        System.out.println(Integer.class.getResourceAsStream("Integer.class") == null);
        // System.out.println(int.class.getResourceAsStream("java/lang/Integer.class") == null); // works in Java 8, but not in Java 21
        System.out.println(int.class.getResourceAsStream("java/") == null);
        // System.out.println(int.class.getResourceAsStream("Test.class") == null); // works in Java 8, but not in Java 21
        System.out.println(Test.class.getResourceAsStream("abcd") == null);
    }
}
