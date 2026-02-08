import java.lang.reflect.Array;

public class Test {
    public static void main(String[] args) throws Exception {
        System.out.println(Array.newInstance(String.class, 0).getClass());
        System.out.println(((String[]) Array.newInstance(String.class, 0)).length);
        System.out.println(Array.newInstance(String.class, 2).getClass());
        System.out.println(((String[]) Array.newInstance(String.class, 2)).length);
        System.out.println(Array.newInstance(String[].class, 4).getClass());
        System.out.println(((String[][]) Array.newInstance(String[].class, 4)).length);
        System.out.println(Array.newInstance(int.class, 3).getClass());
        System.out.println(((int[]) Array.newInstance(int.class, 3)).length);
        System.out.println(Array.newInstance(int[].class, 5).getClass());
        System.out.println(((int[][]) Array.newInstance(int[].class, 5)).length);
        System.out.println(Array.newInstance(float[].class, 5).getClass());
        System.out.println(((float[][]) Array.newInstance(float[].class, 5)).length);
        try {
            Array.newInstance(void.class, 1);
        } catch(IllegalArgumentException e) {
            System.out.println("Got IllegalArgumentException");
        }
        try {
            Array.newInstance(void.class, -1);
        } catch(NegativeArraySizeException e) {
            System.out.println("Got NegativeArraySizeException");
        }
        try {
            Array.newInstance(null, -1);
        } catch(NullPointerException e) {
            System.out.println("Got NullPointerException");
        }
        try {
            Array.newInstance(String.class, -1);
        } catch(NegativeArraySizeException e) {
            System.out.println("Got NegativeArraySizeException");
        }
    }
}
