package java.io;

public class FileOutputStream extends OutputStream {
    private FileDescriptor fd;
    private boolean isOpen;

    public FileOutputStream(String path) throws FileNotFoundException {
        this(new File(path));
    }

    public FileOutputStream(File file) throws FileNotFoundException {
        FileDescriptor fd = FileDescriptor.writeableFromFile(file);

        this.fd = fd;
        this.isOpen = true;
    }

    public FileOutputStream(FileDescriptor fd) {
        this.fd = fd;
        if (fd.valid()) {
            this.isOpen = true;
        } else {
            this.isOpen = false;
        }
    }

    public void write(int b) throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        this.writeInternal(b);
    }

    private native void writeInternal(int b);

    public void flush() throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        this.flushInternal();
    }

    private native void flushInternal();

    public final FileDescriptor getFD() throws IOException {
        return this.fd;
    }
}
