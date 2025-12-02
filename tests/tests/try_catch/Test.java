public class Test {
    public static void main(String[] args) {
        try {
            System.out.println("about to throw throwable");
            throw new Throwable();
        } catch(Throwable e) {
            System.out.println("caught throwable");
        }
        try {
            System.out.println("about to throw throwable");
            try {
                throw new Throwable();
            } catch(Throwable e) {
                System.out.println("caught throwable-1 (1)");
            }
        } catch(Throwable e) {
            System.out.println("caught throwable-2 (1)");
        }
        try {
            System.out.println("about to throw error");
            try {
                throw new Error();
            } catch(Error e) {
                System.out.println("caught error (2)");
            }
        } catch(Throwable e) {
            System.out.println("caught throwable (2)");
        }
        try {
            System.out.println("about to throw error (2)");
            try {
                throw new Error();
            } catch(Throwable e) {
                System.out.println("caught throwable (3)");
            }
        } catch(Error e) {
            System.out.println("caught error (3)");
        }
        try {
            System.out.println("about to throw error (3)");
            throw new Error();
        } catch(Error e) {
            System.out.println("caught error (4)");
        } catch(Throwable e) {
            System.out.println("caught throwable (4)");
        }
    }
}
