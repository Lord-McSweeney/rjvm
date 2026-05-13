public class Test {
    public static void main(String[] args) throws Exception {
        long[] array = new long[]{4L, 5L};
        System.out.println(func(array, 1, 3L));
        System.out.println(array[0]);
        System.out.println(array[1]);
    }
    public static long func(long[] array, int idx, long value) {
        return array[idx] = value;
    }
}
