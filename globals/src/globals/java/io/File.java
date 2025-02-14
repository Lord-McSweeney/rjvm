package java.io;

public class File {
    // TODO get this from system properties
    public static final char separatorChar = '/';

    private String normalizedPath;
    private boolean exists;

    public File(String name) {
        // TODO implement with FileDescriptor?
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
            path = parent + File.separatorChar + child;
        }

        this.internalInitFileData(PrintStream.stringToUtf8(path));
    }

    public boolean exists() {
        return this.exists;
    }

    public native String getAbsolutePath();

    public File getAbsoluteFile() {
        return new File(this.getAbsolutePath());
    }

    public native String getCanonicalPath() throws IOException;

    public String getName() {
        int separatorIndex = this.normalizedPath.lastIndexOf(File.separatorChar);
        return this.normalizedPath.substring(separatorIndex + 1);
    }

    public String getParent() {
        int separatorIndex = this.normalizedPath.lastIndexOf(File.separatorChar);
        if (separatorIndex < 0) {
            return null;
        }
        return this.normalizedPath.substring(0, separatorIndex);
    }

    public String getPath() {
        return this.normalizedPath;
    }

    private native void internalInitFileData(byte[] name);
}
