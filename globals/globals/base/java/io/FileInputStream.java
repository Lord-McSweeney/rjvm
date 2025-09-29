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

    // `available` implementation
    public int available() throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        return this.availableInternal();
    }
    private native int availableInternal();

    // `read()` implementation
    public int read() throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        return this.readInternal();
    }
    private native int readInternal();

    // `read(byte[], int, int)` implementation
    public int read(byte[] b, int offset, int length) throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        if (offset < 0 || length < 0 || offset + length > b.length) {
            throw new IndexOutOfBoundsException();
        }

        // We know `b` is non-null, we just checked `b.length`

        return this.readMultiInternal(b, offset, length);
    }
    private native int readMultiInternal(byte[] b, int offset, int length);

    public final FileDescriptor getFD() throws IOException {
        return this.fd;
    }
}
