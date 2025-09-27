public class Test {
    public static void main(String[] args) {
        String[] strings = new String[]{"hi", "hello world", " ", "abc  def", " ab ", "", "   ", "a b c  d", "   l", "  l", "l  ", "l "};
        for (int i = 0; i < strings.length; i ++) {
            String[] splits = strings[i].split(" ");
            System.out.println(splits.length);
            for (int j = 0; j < splits.length; j ++) {
                System.out.println(splits[j]);
            }
        }
    }
}
