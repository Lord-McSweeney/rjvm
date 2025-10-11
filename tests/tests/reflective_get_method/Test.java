import java.lang.reflect.Method;

public class Test {
    private void privMethod() { }

    protected void protMethod() { }

    public void pubMethod() { }

    public int pubMethodWithArgs(int a, Integer b, Object c) {
        return 1;
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
    }
    
    static void getMethod(Class<?> clazz, String name, Class<?>[] args) {
        try {
            Method result = clazz.getMethod(name, args);
            System.out.println(name + " was found with argc " + result.getParameterCount());
        } catch(Exception e) {
            System.out.println(e.getClass() + " while trying to find " + name);
        }
    }
}
