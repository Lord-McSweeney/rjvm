import java.util.ArrayList;

class NonPublic { }

enum EnumClass { Variant1, Variant2 }

public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(Test.class.getResourceAsStream("Test.class").read());
        System.out.println(Class.forName("[[[LTest;").getResourceAsStream("Test.class").read());
        Class[] classes = new Class[]{Test.class, Comparable.class, System.class, ArrayList.class, NonPublic.class, Class.forName("[B"), Class.forName("[[S"), Class.forName("[LTest;"), double.class, Integer.class, EnumClass.class, Integer[].class, int.class, ArrayList[].class, NonPublic[].class, Comparable[].class, EnumClass[].class};
        for (int i = 0; i < classes.length; i ++) {
            Class klass = classes[i];
            System.out.println(klass.isArray());
            System.out.println(klass.getComponentType());
            System.out.println(klass.isPrimitive());
            System.out.println(klass.isInterface());
            System.out.println(klass.isEnum());
            System.out.println(klass.getEnumConstants() == null);
            System.out.println((klass.getEnumConstants() != null) ? klass.getEnumConstants()[0].getClass() : "...");
            System.out.println(klass.getName());
            System.out.println(klass.getSimpleName());
            System.out.println(klass.getClass().getName());
            System.out.println(klass.getSuperclass());
            System.out.println(klass.getModifiers());
            System.out.println(klass);
        }
    }
}
