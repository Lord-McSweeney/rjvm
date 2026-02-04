public class Test {
    public static void main(String[] args) {
        String[] tests = new String[]{null, "", "-", "+", "0", "a", "A", "C", "4", "+A", "2", "-A", "-+", "-0", "+0", "-AC", "-Z8", "48", "x48", "aVbB7e", "+aVbB7e", "-aVbB7e", "-aaaaaaaaaaa", "-aBcDeFgHiJ", "aBBBCCCddd387", "7fffffff", "80000000", "80000001", "-80000000", "-80000001", "2sb6cs7"};
        int[] radixes = new int[]{0, 1, 2, 3, 6, 8, 10, 12, 16, 20, 24, 28, 32, 36, 37};
        for (int i = 0; i < radixes.length; i ++) {
            for (int j = 0; j < tests.length; j ++) {
                int radix = radixes[i];
                String test = tests[j];

                System.out.print(test + ";" + radix + ":");

                try {
                    System.out.println(Integer.parseInt(test, radix));
                } catch(NumberFormatException e) {
                    System.out.println(e.getClass().getName());
                }
            }
        }
    }
}
