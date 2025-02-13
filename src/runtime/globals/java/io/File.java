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

    public File(String parent, String child) {
        if (child == null) {
            throw new NullPointerException();
        }

        String path;
        if (parent == null) {
            path = child;
        } else if (parent == "") {
            // FIXME this should prepend CWD
            path = child;
        } else {
            // FIXME this is stupid
            path = parent + "/" + child;
        }

        this.internalInitFileData(PrintStream.stringToUtf8(path));
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
