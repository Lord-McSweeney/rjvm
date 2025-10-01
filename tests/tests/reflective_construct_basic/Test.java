public class Test {
    private int i;
    private Object o;

    public Test(int i, Object o) {
        this.i = i;
        this.o = o;
    }

    public static void main(String[] args) {
        System.out.println(Test.class.getConstructors().length);
        try {
            Test instance = (Test) Test.class.getConstructors()[0].newInstance(3, Test.class);
            System.out.println(instance.i);
            System.out.println(instance.o);
        } catch(Exception e) {
            System.out.println("other exception");
        }
    }
}
