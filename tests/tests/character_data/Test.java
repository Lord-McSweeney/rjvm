public class Test {
    public static void main(String[] args) {
        for (char c = 0; c < 256; c ++) {
            System.out.print((int) c + ".");
            System.out.print(Character.getType(c) + ".");
            System.out.print(Character.isLowerCase(c) + ".");
            System.out.print(Character.isUpperCase(c) + ".");
            System.out.print(Character.isDigit(c) + ".");
            System.out.print(Character.isLetter(c) + ".");
            System.out.print(Character.isWhitespace(c) + ".");
            System.out.print(Character.isSpace(c) + ".");
            System.out.print(Character.isDefined(c) + ".");
            for (int i = 0; i < 40; i ++) {
                int digit = Character.digit(c, i);
                if (digit != -1) {
                    System.out.print(digit);
                }
            }
            System.out.println();
        }
    }
}
