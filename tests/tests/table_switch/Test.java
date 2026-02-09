public class Test {
    public static void main(String[] args) {
        doSwitch(3);
        doSwitch(-1);
        doSwitch(6);
        doSwitch(Integer.MIN_VALUE);
        doSwitch(Integer.MAX_VALUE);
    }
    
    static void doSwitch(int f) {
        switch(f) {
            case 2:
                System.out.println(2);
                break;
            case 3:
                System.out.println(3);
                break;
            case 4:
                System.out.println(4);
                break;
            case 5:
                System.out.println(5);
                break;
            default:
                System.out.println(6);
                break;
        }
    }
}
