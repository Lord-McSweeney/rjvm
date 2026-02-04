public class Test {
    public static void main(String[] args) {
        Object[][] theArray = new Object[3][3];
        theArray[1] = new String[2];
        System.out.println(theArray.length);
        System.out.println(theArray[1].length);
    }
}
