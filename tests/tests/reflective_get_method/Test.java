import java.lang.reflect.Method;

public class Test {
    private void privMethod() { }

    protected void protMethod() { }

    public void pubMethod() { }

    public int pubMethodWithArgs(int a, Integer b, Object c) {
        return 1;
    }

    public void overloadMethod() { }

    public Comparable overloadMethod(Comparable a, Object b) {
        return null;
    }

    public String overloadMethod(int[] a, Class[] b) {
        return "hi";
    }

    public static void main(String[] args) throws Exception {
        getMethod(Test.class, "privMethod", new Class<?>[0]);
        getMethod(Test.class, "privMethod", null);
        getMethod(Test.class, "protMethod", new Class<?>[0]);
        getMethod(Test.class, "pubMethod", new Class<?>[0]);
        getMethod(Test.class, "pubMethod", null);
        getMethod(Test.class, "pubMethodWithArgs", new Class<?>[]{int.class, Integer.class, Object.class});
        getMethod(Test.class, "pubMethodWithArgs", new Class<?>[]{Integer.class, Integer.class, Object.class});
        getMethod(Test.class, "pubMethodWithArgs", new Class<?>[]{Integer.class, int.class, Object.class});
        getMethod(Test.class, "clone", new Class<?>[0]);
        getMethod(Class.forName("[I"), "clone", new Class<?>[0]);
        getMethod(Test.class, "hashCode", new Class<?>[0]);
        getMethod(Class.forName("[I"), "hashCode", new Class<?>[0]);
        getMethod(Test.class, "main", new Class<?>[]{Class.forName("[Ljava.lang.String;")});
        getMethod(Test.class, "getMethod", new Class<?>[]{Class.forName("java.lang.Class"), Class.forName("java.lang.String"), Class.forName("[Ljava.lang.Class;")});
        getMethod(ClassLoader.getSystemClassLoader().getClass(), "getSystemClassLoader", new Class<?>[0]);
        getMethod(Comparable.class, "compareTo", new Class<?>[]{Class.forName("java.lang.Object")});
        getMethod(Test.class, "overloadMethod", null);
        getMethod(Test.class, "overloadMethod", new Class<?>[0]);
        getMethod(Test.class, "overloadMethod", new Class<?>[]{Comparable.class, Object.class});
        getMethod(Test.class, "overloadMethod", new Class<?>[]{int[].class, Class[].class});
        getMethod(Test.class, "<init>", null);
        getMethod(Test.class, "<clinit>", null);
    }
    
    static void getMethod(Class<?> clazz, String name, Class<?>[] args) {
        try {
            Method result = clazz.getMethod(name, args);
            System.out.println(name + " was found with argc " + result.getParameterCount());

            Class[] params = result.getParameterTypes();
            for (int i = 0; i < params.length; i ++) {
                System.out.println("    " + params[i]);
            }
        } catch(Exception e) {
            System.out.println(e.getClass() + " while trying to find " + name);
        }
    }
}
