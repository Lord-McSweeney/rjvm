import java.util.EnumSet;

enum EnumClass {
    Variant1,
    Variant2,
    Variant3
}

public class Test {
    public static void main(String[] args) throws Exception {
        EnumSet<EnumClass> all = EnumSet.allOf(EnumClass.class);
        for (EnumClass v : all) {
            System.out.println(v);
        }
        EnumSet<EnumClass> none = EnumSet.noneOf(EnumClass.class);
        for (EnumClass v : none) {
            System.out.println(v);
        }
    }
}
