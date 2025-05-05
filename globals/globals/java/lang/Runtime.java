package java.lang;

public class Runtime {
    private static Runtime theRuntime = new Runtime();

    private Runtime() { }

    public static Runtime getRuntime() {
        return Runtime.theRuntime;
    }

    public native void exit(int status);

    public void addShutdownHook(Thread hook) {
        // TODO implement
    }
}
