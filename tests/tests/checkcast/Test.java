import java.util.*;

public class Test {
    public static void main(String[] args) {
        System.out.println(((Comparable) (new Integer(4))).getClass());
        System.out.println(((Integer) (new Integer(4))).getClass());
        System.out.println(((Object) (new Integer(4))).getClass());
        System.out.println(((Map<Integer, Integer>) (new HashMap<Integer, Integer>())).getClass());
        try {
            System.out.println(((Test) (Object) (new Integer(4))).getClass());
        } catch(Exception e) {
            System.out.println(e.getClass());
        }
        System.out.println(((Object[]) (new Integer[4])).getClass());
        System.out.println(((Integer[]) (new Integer[4])).getClass());
        System.out.println(((Object[]) (new Object[4])).getClass());
        try {
            System.out.println(((Integer[]) (new Object[4])).getClass());
        } catch(Exception e) {
            System.out.println(e.getClass());
        }
        try {
            System.out.println(((Object[]) (Object) (new int[4])).getClass());
        } catch(Exception e) {
            System.out.println(e.getClass());
        }
        try {
            System.out.println(((Test[]) (new Test[4])).getClass());
        } catch(Exception e) {
            System.out.println(e.getClass());
        }
        System.out.println(((Object[][]) (new Integer[4][3])).getClass());
        System.out.println(((Integer[][]) (new Integer[4][3])).getClass());
        System.out.println(((Object[][]) (new Object[4][3])).getClass());
        try {
            System.out.println(((Integer[][]) (new Object[4][3])).getClass());
        } catch(Exception e) {
            System.out.println(e.getClass());
        }
        System.out.println(((Test[][]) (new Test[4][3])).getClass());
        System.out.println(((Object[]) (new int[4][3])).getClass());
        try {
            System.out.println(((Object[][]) (Object) (new Test[3])).getClass());
        } catch(Exception e) {
            System.out.println(e.getClass());
        }
    }
}
