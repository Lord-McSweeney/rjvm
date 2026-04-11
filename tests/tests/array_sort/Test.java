import java.util.Arrays;

public class Test {
    public static void main(String[] args) {
        String[] array1 = new String[]{"hi", "ab", "", "abc"};
        String[] array2 = new String[]{"b", "d", "c", "a"};
        Integer[] array3 = new Integer[]{5, 6, 3, 2};
        String[] array4 = new String[]{};

        Arrays.sort(array1, 1, 3);
        printArray(array1);

        Arrays.sort(array2);
        printArray(array2);

        Arrays.sort(array3, 1, 3);
        printArray(array3);

        Arrays.sort(array4);
        printArray(array4);
    }
    
    public static void printArray(Object[] arr) {
        for (Object o : arr) System.out.print(o + " ");
        System.out.println();
    }
}
