public class Test {
    public static void main(String[] args) {
        Class[] classes = new Class[]{Byte.class, Character.class, Boolean.class, Short.class, Integer.class, Double.class, Long.class, Float.class, Void.class};
        for (int i = 0; i < classes.length; i ++) {
            Class klass = classes[i];
            System.out.println(klass);
            System.out.println(klass.getName());
            System.out.println(klass.isInterface());
            System.out.println(klass.isPrimitive());
        }

        Class[] primClasses = new Class[]{Byte.TYPE, Character.TYPE, Boolean.TYPE, Short.TYPE, Integer.TYPE, Double.TYPE, Long.TYPE, Float.TYPE, Void.TYPE};
        for (int i = 0; i < primClasses.length; i ++) {
            Class klass = primClasses[i];
            System.out.println(klass);
            System.out.println(klass.getName());
            System.out.println(klass.isInterface());
            System.out.println(klass.isPrimitive());
        }
    }
}
