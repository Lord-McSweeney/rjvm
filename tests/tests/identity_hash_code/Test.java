class CustomHashCode {
    public int hashCode() {
        return super.hashCode() - 1;
    }
}
public class Test {
    public static void main(String[] args) {
        CustomHashCode i1 = new CustomHashCode();
        int assocHashCode = i1.hashCode();
        int identHashCode = System.identityHashCode(i1);
        System.out.println(assocHashCode == identHashCode);
        System.out.println(System.identityHashCode(null));
    }
}
