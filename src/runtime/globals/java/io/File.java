package java.io;

public class File {
    private byte[] name;
    private boolean exists;

    public File(String name) {
        // TODO implement with FileDescriptor
        // Currently we just initialize all properties in an `internalInitFromName`
        // because File is immutable
        this.internalInitFileData(PrintStream.stringToUtf8(name));
    }

    public boolean exists() {
        return this.exists;
    }

    public native String getCanonicalPath() throws IOException;

    public native String getName();

    public native String getParent();

    public native String getPath();

    private native void internalInitFileData(byte[] name);
}
