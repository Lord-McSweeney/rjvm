public class Test {
    public static void main(String[] args) {
        StringBuilder builder = new StringBuilder("initial");
        builder.append('c');
        builder.append(14);
        builder.append(6L);
        builder.append("hi");
        builder.append((Object) null);
        builder.append((String) null);
        builder.append(new char[]{'a', 'b'});
        builder.append(Test.class);
        System.out.println(builder);
    }
}
