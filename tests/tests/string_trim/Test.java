public class Test {
    public static void main(String[] args) {
        String[] strings = new String[]{".", "", " ", "\r", "\03", " \03 ", "    ", "'  ", "   '", "!", "\r\r \n", "\r4", "\t77", "asd vdfv   siudhf", "   asiudh   iuh  oih\r"};
        for (String str : strings) {
            testTrim(str);
        }
    }

    static void testTrim(String str) {
        System.out.println(str);
        System.out.println("    " + str.trim());
        System.out.println("    " + str.trim().length());
    }
}
