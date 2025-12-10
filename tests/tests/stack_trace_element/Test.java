public class Test {
    public static void main(String[] args) {
        try {
            new Test().aMethod();
        } catch(Error err) {
            StackTraceElement[] elems = err.getStackTrace();
            for (StackTraceElement elem : elems) {
                System.out.println(elem.getClassName());
                System.out.println(elem.getMethodName());
            }
        }
    }

    public void aMethod() {
        this.bMethod();
    }

    public void bMethod() {
        throw new Error();
    }
}
