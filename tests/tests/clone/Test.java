import java.util.Arrays;

class SimpleObj implements Cloneable {
    public int value;

    public String toString() {
        return "obj(" + Integer.toString(this.value) + ")";
    }

    public Object clone() {
        try {
            return super.clone();
        } catch(Exception e) {
            throw new Error();
        }
    }
}

public class Test {
    public static void main(String[] args) {
        new Test().run();
    }

    public void run() {
        SimpleObj obj = new SimpleObj();
        obj.value = 4;
        System.out.println(obj);
        obj.value = 5;
        System.out.println(obj);
        SimpleObj obj2 = (SimpleObj) obj.clone();
        System.out.println(obj);
        System.out.println(obj2);
        obj2.value = 6;
        System.out.println(obj);
        System.out.println(obj2);
        obj.value = 7;
        System.out.println(obj);
        System.out.println(obj2);

        SimpleObj[] objArray = new SimpleObj[]{null, obj2};
        objArray[0] = obj;
        System.out.println(Arrays.toString(objArray));
        SimpleObj[] objArray2 = objArray.clone();
        System.out.println(Arrays.toString(objArray));
        System.out.println(Arrays.toString(objArray2));
        objArray[0] = null;
        System.out.println(Arrays.toString(objArray));
        System.out.println(Arrays.toString(objArray2));
        obj2.value = 8;
        System.out.println(Arrays.toString(objArray));
        System.out.println(Arrays.toString(objArray2));
        
        int[] intArray = new int[]{9, 10};
        System.out.println(intArray[0]);
        int[] intArray2 = intArray.clone();
        intArray[0] = 5;
        System.out.println(intArray[0]);
        System.out.println(intArray2[0]);

        try {
            this.clone();
        } catch (CloneNotSupportedException e) {
            System.out.println("CloneNotSupportedException");
        }
    }
}
