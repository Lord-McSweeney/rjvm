import java.lang.reflect.Field;

class Abc {
    private long field1;
    public Object fromAbc;
    public static Class fromAbcStatic;
}

public class Test extends Abc {
    private int field1;
    private Test field2;
    private String field3;

    protected Object field4;
    protected static int field5;

    public static long field6;
    public float field7;
    public Comparable field8;
    public long field9;

    public static void main(String[] args) {
        Field[] array = Test.class.getDeclaredFields();
        sortFieldsArray(array);
        System.out.println(array.length);
        for (Field field : array) {
            System.out.println(field.getName());
            System.out.println("    " + field.getDeclaringClass());
            System.out.println("    " + field.getType());
            System.out.println("    " + field.getModifiers());

            try {
                System.out.println("    Read with get(null): " + field.get(null));
            } catch(Exception e) {
                System.out.println("    Read with get(null): error " + e.getClass());
            }
            try {
                System.out.println("    Read with get(Test.class): " + field.get(Test.class));
            } catch(Exception e) {
                System.out.println("    Read with get(Test.class): error " + e.getClass());
            }
            try {
                System.out.println("    Read with get(new Test()): " + field.get(new Test()));
            } catch(Exception e) {
                System.out.println("    Read with get(new Test()): error " + e.getClass());
            }
        }
    }
    
    public static void sortFieldsArray(Field[] fields) {
        boolean changed = true;
        while (changed) {
            changed = false;
            for (int i = 0; i < fields.length - 1; i ++) {
                if (fields[i].getName().compareTo(fields[i + 1].getName()) > 0) {
                    Field temp = fields[i];
                    fields[i] = fields[i + 1];
                    fields[i + 1] = temp;
                    changed = true;
                }
            }
        }
    }
}
