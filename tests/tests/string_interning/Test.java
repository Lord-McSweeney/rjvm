public class Test {
    public static void main(String[] args) {
        String hiString = "hi";
        String newString = new String("hi");
        System.out.println(hiString);
        System.out.println(hiString.intern());
        System.out.println(hiString.intern() == hiString);
        System.out.println(newString.intern() == hiString);
        System.out.println(newString.intern() == newString);
        System.out.println(hiString == newString);
        System.out.println("".intern());
        System.out.println((new String(newString)).intern() == hiString);
    }
}
