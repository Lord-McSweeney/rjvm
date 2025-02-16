package java.io;

public class FileInputStream extends InputStream {
    private FileDescriptor fd;
    private boolean isOpen;

    public FileInputStream(String fileName) throws FileNotFoundException {
        this(new File(fileName));
    }

    public FileInputStream(File file) throws FileNotFoundException {
        FileDescriptor fd = FileDescriptor.readableFromFile(file);

        this.fd = fd;
        this.isOpen = true;
    }

    public FileInputStream(FileDescriptor fd) {
        this.fd = fd;
        if (fd.valid()) {
            this.isOpen = true;
        } else {
            this.isOpen = false;
        }
    }

    public int read() throws IOException {
        // TODO implement
        return -1;
    }
}
