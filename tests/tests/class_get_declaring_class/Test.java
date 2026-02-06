import java.util.Map;

public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(Test.class.getDeclaringClass());
        System.out.println(int[].class.getDeclaringClass());
        System.out.println(Test[].class.getDeclaringClass());
        System.out.println(Map.class.getDeclaringClass());
        System.out.println(Map.Entry.class.getDeclaringClass());
        System.out.println(Map.Entry[].class.getDeclaringClass());
        System.out.println(InnerClass.class.getDeclaringClass());
        System.out.println(Test.InnerClass.class.getDeclaringClass());
    }
    
    static class InnerClass { }
}
