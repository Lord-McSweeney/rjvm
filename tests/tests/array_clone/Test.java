public class Test {
    public static void main(String[] args) {
        int[] intArray = new int[]{1, 4, 5, 2};
        int[] intArrayCloned = intArray.clone();
        for (int i = 0; i < intArrayCloned.length; i ++) {
            System.out.println(intArrayCloned[i]);
        }
        System.out.println(intArrayCloned.length);
        System.out.println(intArrayCloned.getClass());

        String[] stringArray = new String[]{"hi", "hello", null, ""};
        String[] stringArrayCloned = stringArray.clone();
        for (int i = 0; i < stringArrayCloned.length; i ++) {
            System.out.println(stringArrayCloned[i]);
        }
        System.out.println(stringArrayCloned.length);
        System.out.println(stringArrayCloned.getClass());

        Object[] objectStringArray = (Object[]) stringArray;
        Object[] objectStringArrayCloned = objectStringArray.clone();
        for (int i = 0; i < objectStringArrayCloned.length; i ++) {
            System.out.println(objectStringArrayCloned[i]);
        }
        System.out.println(objectStringArrayCloned.length);
        System.out.println(objectStringArrayCloned.getClass());

        boolean[] boolArray = new boolean[]{true, false, false};
        boolean[] boolArrayCloned = boolArray.clone();
        for (int i = 0; i < boolArrayCloned.length; i ++) {
            System.out.println(boolArrayCloned[i]);
        }
        System.out.println(boolArrayCloned.length);
        System.out.println(boolArrayCloned.getClass());
        
        System.out.println((new float[0]).clone().length);
        System.out.println((new float[0]).clone().getClass());

        try {
            System.out.println(int[].class.getClass().getMethod("clone"));
        } catch(NoSuchMethodException e) {
            System.out.println("Caught NSME");
        }
    }
}
