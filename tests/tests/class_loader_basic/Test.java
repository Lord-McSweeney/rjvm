import java.io.*;
import java.nio.*;
import java.util.*;

public class Test {
    public static void main(String[] args) throws Throwable {
        printCL(String.class.getClassLoader());
        printCL(Test.class.getClassLoader());
        printCL(Test.class.getClassLoader().getParent());
        printCL(Test.class.getClassLoader().getParent().getParent());
        printCL(Class.forName("[LTest;").getClassLoader());
        printCL(Class.forName("[[[[LTest;").getClassLoader());
        printCL(Class.forName("[[[I").getClassLoader());
        printCL(Class.forName("[Ljava.lang.Integer;").getClassLoader());
        printCL(Class.forName("[Ljava.lang.Integer;").getClassLoader());
        printCL(ClassLoader.getSystemClassLoader());
        printCL(ClassLoader.getSystemClassLoader().getParent());
    }

    static void printCL(ClassLoader loader) {
        if (loader == null) {
            System.out.println("null");
        } else if (loader == ClassLoader.getSystemClassLoader()) {
            System.out.println("system loader");
        } else if (loader == ClassLoader.getSystemClassLoader().getParent()) {
            System.out.println("platform loader");
        } else {
            System.out.println("other loader");
        }
    }
}
