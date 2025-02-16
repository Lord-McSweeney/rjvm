package java.io;

public final class FileDescriptor {
    public static FileDescriptor in = FileDescriptor.fromDescriptor(0);
    public static FileDescriptor out = FileDescriptor.fromDescriptor(1);
    public static FileDescriptor err = FileDescriptor.fromDescriptor(2);

    private int descriptor;

    public FileDescriptor() {
        this.descriptor = -1;
    }

    public boolean valid() {
        return this.descriptor != -1;
    }

    private static FileDescriptor fromDescriptor(int descriptor) {
        FileDescriptor fd = new FileDescriptor();
        fd.descriptor = descriptor;
        return fd;
    }

    static FileDescriptor fromFile(File file) throws FileNotFoundException {
        int registeredDescriptor = FileDescriptor.internalDescriptorFromPath(file.getPath());
        // -1 signals an error
        if (registeredDescriptor == -1) {
            throw new FileNotFoundException();
        }

        FileDescriptor fd = new FileDescriptor();
        fd.descriptor = registeredDescriptor;
        return fd;
    }

    private static native int internalDescriptorFromPath(String filePath);
}
