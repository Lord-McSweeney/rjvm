public class Test {
    public Test(int i) {
        super();
    }

    protected Test(String s) {
        super();
    }

    public static void main(String[] args) {
        System.out.println(Test.class.getConstructors().length);
        System.out.println(Integer.class.getConstructors().length);
        System.out.println(System.class.getConstructors().length);
    }
}
