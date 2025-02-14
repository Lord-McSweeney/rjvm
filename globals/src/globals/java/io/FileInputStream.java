package java.io;

public class FileInputStream extends InputStream {
    public FileInputStream(String fileName) {
        this(new File(fileName));
    }

    public FileInputStream(File file) {
        // TODO implement
        super();
    }

    public int read() throws IOException {
        // TODO implement
        return -1;
    }
}
