public class Test {
    public static void main(String[] args) throws Throwable {
        int[][][] arr = new int[3][4][5];
        System.out.println(arr.getClass());
        System.out.println(arr.length);
        System.out.println(arr[0].length);
        System.out.println(arr[0].getClass());
        System.out.println(arr[0][0].length);
        System.out.println(arr[0][0].getClass());
        System.out.println(arr[0][0][0]);

        byte[][][] arr1 = new byte[3][4][5];
        System.out.println(arr1.getClass());
        System.out.println(arr1.length);
        System.out.println(arr1[0].length);
        System.out.println(arr1[0].getClass());
        System.out.println(arr1[0][0].length);
        System.out.println(arr1[0][0].getClass());
        System.out.println(arr1[0][0][0]);

        long[][][] arr2 = new long[3][4][5];
        System.out.println(arr2.getClass());
        System.out.println(arr2.length);
        System.out.println(arr2[0].length);
        System.out.println(arr2[0].getClass());
        System.out.println(arr2[0][0].length);
        System.out.println(arr2[0][0].getClass());
        System.out.println(arr2[0][0][0]);

        Object[][][] arr3 = new Object[3][4][5];
        System.out.println(arr3.getClass());
        System.out.println(arr3.length);
        System.out.println(arr3[0].length);
        System.out.println(arr3[0].getClass());
        System.out.println(arr3[0][0].length);
        System.out.println(arr3[0][0].getClass());
        System.out.println(arr3[0][0][0]);
    }
}
