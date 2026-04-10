class ClassLoader1 extends ClassLoader {
    public ClassLoader1() {
        super(null);
    }
}

class ClassLoader2 extends ClassLoader {
    public ClassLoader2() {
        super(null);
    }
    
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        System.out.println("        CL2: findClass on " + name);
        return null;
    }
}

class ClassLoader3 extends ClassLoader {
    public ClassLoader3() {
        super(ClassLoader.getSystemClassLoader());
    }
    
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        System.out.println("        CL3: findClass on " + name);
        if (name == null) {
            return ClassLoader3.class;
        } else {
            return null;
        }
    }
}

class ClassLoader4 extends ClassLoader {
    public ClassLoader4() {
        super(ClassLoader.getSystemClassLoader());
    }
    
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        System.out.println("        CL4: findClass on " + name);
        throw new ClassNotFoundException();
    }
}

class ClassLoader5 extends ClassLoader {
    public ClassLoader5() {
        super(null);
    }
    
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        System.out.println("        CL5: findClass on " + name);
        return ClassLoader5.class;
    }
}

class ClassLoader6 extends ClassLoader {
    public ClassLoader6() {
        super(ClassLoader.getSystemClassLoader());
    }
    
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        System.out.println("        CL6: findClass on " + name);
        return ClassLoader6.class;
    }
}

public class Test {
    public static void main(String[] args) {
        ClassLoader[] loaders = new ClassLoader[]{new ClassLoader1(), new ClassLoader2(), new ClassLoader3(), new ClassLoader4(), new ClassLoader5(), new ClassLoader6()};

        for (ClassLoader loader : loaders) {
            System.out.println(loader.getClass());
            try {
                System.out.println("    " + loader.loadClass(null));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("Hello"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("/"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("."));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("Test"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass(".Test"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("java.lang.Integer"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("java/lang/Integer"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("ClassLoader1"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("[LClassLoader1;"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("[I"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("[V"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("[[I"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("[Ljava/lang/Integer;"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
            try {
                System.out.println("    " + loader.loadClass("[invalidarraydesc"));
            } catch(Exception e) {
                System.out.println("    " + e);
            }
        }
    }
}
