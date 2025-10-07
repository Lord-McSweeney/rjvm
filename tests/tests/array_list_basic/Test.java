import java.util.ArrayList;

public class Test {
    public static void main(String[] args) {
        ArrayList<Integer> list = new ArrayList<Integer>();
        list.add(2);
        list.add(6);
        list.add(3);
        list.add(-652);
        list.add(289);
        System.out.println(list);
        System.out.println(list.size());
        list.remove(4);
        System.out.println(list);
        System.out.println(list.size());
        list.add(0, 19);
        list.add(list.size(), 2);
        System.out.println(list);
        System.out.println(list.size());
        for (int i = 0; i < list.size(); i ++) {
            System.out.println(list.get(i));
        }
        try {
            System.out.println(list.get(-1));
        } catch(RuntimeException e) {
            System.out.println("Caught exception");
        }
        try {
            System.out.println(list.get(list.size()));
        } catch(RuntimeException e) {
            System.out.println("Caught exception");
        }
        for (int elem : list) {
            System.out.println(elem);
        }
        ArrayList<Integer> list2 = new ArrayList<Integer>(0);
        list2.add(19);
        System.out.println(list2);
    }
}
