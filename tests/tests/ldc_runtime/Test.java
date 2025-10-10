public class Test {
    public static void main(String[] args) {
        char[] arr = new char[]{'h', 'i'};
        String hiStr = new String(arr);
        System.out.println(hiStr.intern() == hiStr);
        System.out.println("hi");
    }
}
