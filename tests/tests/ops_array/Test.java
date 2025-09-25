public class Test {
    public static void main(String[] args) {
        long[] longArray = new long[5];
        longArray[0] = 1;
        longArray[1] = 5;
        longArray[2] = 0;
        System.out.println(longArray[1]);
        System.out.println(longArray.length);
        try {
            longArray[-1] = 40000;
        } catch(ArrayIndexOutOfBoundsException e) {
            System.out.println("caught ArrayIndexOutOfBoundsException");
        }
        try {
            longArray[5] = -20000;
        } catch(ArrayIndexOutOfBoundsException e) {
            System.out.println("caught ArrayIndexOutOfBoundsException");
        }
        try {
            System.out.println(longArray[-1]);
        } catch(ArrayIndexOutOfBoundsException e) {
            System.out.println("caught ArrayIndexOutOfBoundsException");
        }
        try {
            System.out.println(longArray[5]);
        } catch(ArrayIndexOutOfBoundsException e) {
            System.out.println("caught ArrayIndexOutOfBoundsException");
        }
        long[] nullArray = null;
        try {
            System.out.println(nullArray[0]);
        } catch(NullPointerException e) {
            System.out.println("caught NullPointerException");
        }
        try {
            System.out.println(nullArray.length);
        } catch(NullPointerException e) {
            System.out.println("caught NullPointerException");
        }
        try {
            nullArray[100] = 4;
        } catch(NullPointerException e) {
            System.out.println("caught NullPointerException");
        }
        for (int i = 0; i < longArray.length; i ++) {
            System.out.println(longArray[i]);
        }
    }
}
