package java.io;

import java.nio.charset.Charset;

public class File {
    // TODO get this from system properties
    public static final char separatorChar = '/';
    public static final String separator = "" + File.separatorChar;

    public static final char pathSeparatorChar = ':';
    public static final String pathSeparator = "" + File.pathSeparatorChar;

    private String normalizedPath;
    private boolean exists;

    public File(String name) {
        // We can initialize all properties in an `internalInitFromName`
        // because File is immutable
        this.internalInitFileData(Charset.stringToUtf8(name));
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

        this.internalInitFileData(Charset.stringToUtf8(path));
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
        if (this.normalizedPath.equals(File.separator)) {
            return null;
        }

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
