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

        this.writeNative(b);
    }

    public void write(byte[] buffer, int ofs, int len) throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        if (ofs < 0 || len < 0 || ofs + len > buffer.length || ofs + len < 0) {
            throw new IndexOutOfBoundsException();
        }

        // We know `b` is non-null, we just checked `b.length`

        this.writeMultipleNative(buffer, ofs, len);
    }

    public void flush() throws IOException {
        if (!this.isOpen) {
            throw new IOException();
        }

        this.flushNative();
    }

    public final FileDescriptor getFD() throws IOException {
        return this.fd;
    }

    // Native methods

    private native void writeNative(int b);

    private native void writeMultipleNative(byte[] buffer, int ofs, int len);

    private native void flushNative();
}
