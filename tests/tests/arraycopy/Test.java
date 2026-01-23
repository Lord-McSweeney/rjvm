public class Test {
    public static void main(String[] args) {
        int[] array1 = new int[]{9, 7, 8, 5, 6, 3, 4, 1};
        int[] array2 = new int[]{21, 34, 65, 78, 98, 12, 45, 89};
        int[] array3 = new int[]{11, 22, 33, 44, 55, 66, 77, 88};
        int[] array4 = new int[]{120000, 340000, 560000, 780000, 210000, 430000, 650000, 870000};
        int[][] arrays = new int[][]{array1, array2, array3, array4};

        for (int n1 = 0; n1 <= 9; n1 ++) {
            for (int n2 = 0; n2 <= 9; n2 ++) {
                for (int n3 = 0; n3 <= 9; n3 ++) {
                    for (int arr1 = 0; arr1 < 4; arr1 ++) {
                        for (int arr2 = 0; arr2 < 4; arr2 ++) {
                            System.out.print(n1 + "," + n2 + "," + n3 + "," + arr1 + "," + arr2 + ":");

                            int[] arraySource = arrays[arr1];
                            int[] arrayDest = arrays[arr2];

                            try {
                                System.arraycopy(arraySource, n1, arrayDest, n2, n3);
                                for (int i = 0; i < arrayDest.length; i ++) {
                                    System.out.print(arrayDest[i] + " ");
                                }
                                System.out.println();
                            } catch(Exception e) {
                                System.out.println(e.getClass().getName());
                            }
                        }
                    }
                }
            }
        }
    }
}
