import java.util.Dictionary;

public class Test {
    private int i;
    private Object o;

    public Test(int i, Object o) {
        this.i = i;
        this.o = o;
    }

    public static void main(String[] args) {
        System.out.println(Test.class.getConstructors().length);
        System.out.println(Test.class.getConstructors()[0].getParameterCount());
        try {
            Test instance = (Test) Test.class.getConstructors()[0].newInstance(3, Test.class);
            System.out.println(instance.i);
            System.out.println(instance.o);
        } catch(Exception e) {
            System.out.println("other exception " + e.getClass());
        }
        System.out.println(Dictionary.class.getConstructors().length);
        System.out.println(Dictionary.class.getConstructors()[0].getParameterCount());
        try {
            Dictionary.class.getConstructors()[0].newInstance();
        } catch(Exception e) {
            System.out.println("exception " + e.getClass());
        }
    }
}
