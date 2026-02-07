import java.util.HashMap;
import java.util.Map;

public class Test {
    public static void main(String[] args) {
        HashMap<Long, String> theMap = new HashMap<Long, String>();
        System.out.println(theMap.put(3L, "hello"));
        System.out.println(theMap.put(1L, "hello"));
        System.out.println(theMap.put(2L, "hi"));
        System.out.println(theMap.put(4L, "iadj"));
        System.out.println(theMap.put(3L, "hi"));
        System.out.println(theMap.put(4L, "9"));

        System.out.println(theMap.size());

        System.out.println(theMap.get(1L));
        System.out.println(theMap.get(2L));
        System.out.println(theMap.get(3L));
        System.out.println(theMap.get(null));
        System.out.println(theMap.get(99L));
        System.out.println(theMap.get(999999999999L));

        boolean foundFirst = false;
        boolean foundSecond = false;
        boolean foundThird = false;
        boolean foundFourth = false;
        boolean foundUnexpected = false;
        for (Map.Entry<Long, String> entry : theMap.entrySet()) {
            if (entry.getKey() == 1L && entry.getValue().equals("hello")) {
                foundFirst = true;
            } else if (entry.getKey() == 2L && entry.getValue().equals("hi")) {
                System.out.println(entry.setValue("ho"));
                foundSecond = true;
            } else if (entry.getKey() == 3L && entry.getValue().equals("hi")) {
                foundThird = true;
            } else if (entry.getKey() == 4L && entry.getValue().equals("9")) {
                foundFourth = true;
            } else {
                foundUnexpected = true;
            }
        }
        System.out.println(foundFirst);
        System.out.println(foundSecond);
        System.out.println(foundThird);
        System.out.println(foundFourth);
        System.out.println(foundUnexpected);

        System.out.println(theMap.get(2L));

        foundFirst = false;
        foundSecond = false;
        foundThird = false;
        foundFourth = false;
        foundUnexpected = false;
        for (Long entry : theMap.keySet()) {
            if (entry == 1L) {
                foundFirst = true;
            } else if (entry == 2L) {
                foundSecond = true;
            } else if (entry == 3L) {
                foundThird = true;
            } else if (entry == 4L) {
                foundFourth = true;
            } else {
                foundUnexpected = true;
            }
        }
        System.out.println(foundFirst);
        System.out.println(foundSecond);
        System.out.println(foundThird);
        System.out.println(foundFourth);
        System.out.println(foundUnexpected);

        System.out.println(theMap.size());
        System.out.println(theMap.containsKey(null));
        System.out.println(theMap.containsKey(0L));
        System.out.println(theMap.containsKey(1L));
        System.out.println(theMap.containsKey(2L));
        System.out.println(theMap.containsKey(3L));
        System.out.println(theMap.containsKey(4L));
        System.out.println(theMap.containsKey(5L));

        System.out.println(theMap.put(null, "f"));
        System.out.println(theMap.get(null));
        System.out.println(theMap.put(null, "g"));

        theMap.put(5L, "5a");
        theMap.put(6L, "6b");
        theMap.put(7L, "7c");
        theMap.put(8L, "8d");
        theMap.put(9L, "9e");
        theMap.put(10L, "10f");
        theMap.put(11L, "11g");

        theMap.clear();

        System.out.println(theMap.get(0L));
        System.out.println(theMap.get(1L));
        System.out.println(theMap.get(2L));
        System.out.println(theMap.get(3L));
        System.out.println(theMap.get(4L));
        System.out.println(theMap.get(5L));
        System.out.println(theMap.get(6L));
        System.out.println(theMap.get(7L));
        System.out.println(theMap.get(8L));
        System.out.println(theMap.get(9L));
        System.out.println(theMap.get(10L));
        System.out.println(theMap.get(11L));
        System.out.println(theMap.get(12L));
    }
}
