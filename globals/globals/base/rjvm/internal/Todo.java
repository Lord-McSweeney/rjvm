package rjvm.internal;

public class Todo {
    public static void warnPartialImpl(String info) {
        System.err.println("(rjvm warning) " + info + " partially implemented");
    }

    public static void warnNotImpl(String info) {
        System.err.println("(rjvm warning) " + info + " not implemented");
    }
}
