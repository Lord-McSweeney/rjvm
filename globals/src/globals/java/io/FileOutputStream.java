package java.io;

public class FileOutputStream extends OutputStream {
    private FileDescriptor fd;
    private boolean isOpen;

    public FileOutputStream(File file) {
        FileDescriptor fd = FileDescriptor.fromFile(file);

        this.fd = fd;
        // TODO implement
        this.isOpen = false;
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

    public final FileDescriptor getFD() throws IOException {
        return this.fd;
    }
}
