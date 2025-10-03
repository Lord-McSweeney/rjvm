public class Test {
    private int fInt;
    private long fLong;
    private Test fObj;
    private float fFloat;
    private Test fObj2;
    private short fShort;
    private char fChar;
    private byte fByte;
    private double fDouble;

    protected int intField;

    public Test(int value) {
        this.intField = value;
    }

    public String toString() {
        return Integer.toString(this.intField);
    }

    public static void main(String[] args) {
        Test test1 = new Test(30);
        Test test2 = new Test(35);
        test1.fInt = 523489;
        test1.fLong = -2987349873L;
        test1.fObj = test1;
        test1.fFloat = 3.875f;
        test1.fObj2 = test2;
        test1.fShort = 3611;
        test1.fChar = '枉';
        test1.fByte = -67;
        test1.fDouble = 1847243.875;
        System.out.println(test1.fInt);
        System.out.println(test1.fLong);
        System.out.println(test1.fObj);
        System.out.println(((int) (test1.fFloat)) * 1000);
        System.out.println(test1.fShort);
        System.out.println((int) test1.fChar);
        System.out.println(test1.fByte);
        System.out.println(((int) (test1.fDouble)) * 1000);
    }
}
