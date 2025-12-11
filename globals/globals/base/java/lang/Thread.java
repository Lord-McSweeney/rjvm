package java.lang;

public class Thread implements Runnable {
    private Runnable target;

    public Thread() { }

    public Thread(Runnable target) {
        this.target = target;
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

    public static void sleep(long millis) throws InterruptedException {
        // TODO implement
    }
}
