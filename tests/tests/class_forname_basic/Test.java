public class Test {
    public static void main(String[] args) {
        String[] arr = new String[]{"java.lang.Class", "java.lang.Integer", "java.lang.NoClassDefFoundError", "java.lang.Comparable", "java.util.ArrayList", "Test", "java/lang/Class", "int", "void", "I", "[I", "[Ljava/lang/String;", "[Ljava.lang.String;", null};

        for (int i = 0; i < arr.length; i ++) {
            try {
                System.out.println(Class.forName(arr[i]));
            } catch(ClassNotFoundException e) {
                System.out.println("not found");
            } catch(NullPointerException e) {
                System.out.println("null passed");
            }
        }
    }
}
