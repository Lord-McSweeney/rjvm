package java.lang;

public class Thread implements Runnable {
    private Runnable target;
    private ClassLoader contextClassLoader;

    public Thread() {
        this.init(null);
    }

    public Thread(Runnable target) {
        this.init(target);
    }

    private void init(Runnable target) {
        this.target = target;
        this.contextClassLoader = ClassLoader.getSystemClassLoader(); // TODO
    }

    public synchronized void start() {
        // TODO: Call the `run` method from a different thread
        this.run();
    }

    public void run() {
        if (this.target != null) {
            this.target.run();
        }
    }

    public ClassLoader getContextClassLoader() {
        return this.contextClassLoader;
    }

    public static void sleep(long millis) throws InterruptedException {
        // TODO implement
    }

    // :P
    private static Thread currentThread;
    public static Thread currentThread() {
        if (Thread.currentThread == null) {
            Thread.currentThread = new Thread();
        }

        return Thread.currentThread;
    }
}
