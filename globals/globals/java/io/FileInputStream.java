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

    public int available() throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        return this.availableInternal();
    }

    public native int availableInternal();

    public int read() throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        return this.readInternal();
    }

    public native int readInternal();
}
