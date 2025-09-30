public class Test {
    public static void main(String[] args) {
        Test.runToStringTests();

        Test.runParseIntTests();
    }

    // `toString` and `toHexString` tests
    static void runToStringTests() {
        int[] arr = new int[]{7, -236, 1787234898, 0, -2147483648, 2147483647, 100000, 0x10000};
        int[] radixes = new int[]{-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 34, 35, 36, 37, 38, 39, 1000};
        for (int i = 0; i < arr.length; i ++) {
            for (int j = 0; j < radixes.length; j ++) {
                System.out.print(arr[i] + "," + radixes[j] + ": ");
                System.out.println(Integer.toString(arr[i], radixes[j]));
            }
            System.out.print(arr[i] + ": ");
            System.out.println(arr[i]);
            System.out.println(Integer.toString(arr[i]));
            System.out.println((new Integer(arr[i])).toString());
            System.out.println(Integer.toHexString(arr[i]));
        }
    }

    // `parseInt` tests
    static void runParseIntTests() {
        String[] arr = new String[]{" ", "", "+", "-", "0", "-0", "--", "++", "++1", "--1", "1", "12378", "89", "-4", "-27348", "-2147483647", "-2147483648", "-2147483649", "2147483646", "2147483647", "2147483648", "2147483650", "-2147483650", "87234832764873264872643873264732648732483264872643873", "-87234832764873264872643873264732648732483264872643873"};
        for (int i = 0; i < arr.length; i ++) {
            try {
                System.out.println(arr[i] + ": " + Integer.parseInt(arr[i]));
            } catch(NumberFormatException e) {
                System.out.println(arr[i] + ": number format exception");
            }
        }
    }
}
