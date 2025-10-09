package java.io;

import rjvm.internal.Todo;

import java.nio.charset.Charset;

public class File {
    public static final String separator = System.getProperty("file.separator");
    public static final char separatorChar = File.separator.charAt(0);

    public static final String pathSeparator = System.getProperty("path.separator");
    public static final char pathSeparatorChar = File.pathSeparator.charAt(0);

    private String normalizedPath;
    private boolean exists;
    private boolean isDirectory;

    public File(String name) {
        // We can initialize all properties in an `internalInitFromName`
        // because File is immutable
        this.internalInitFileData(Charset.stringToUtf8(name));
    }

    public File(File parent, String child) {
        if (child == null) {
            throw new NullPointerException();
        }

        String path;
        if (parent == null) {
            path = child;
        } else if (parent.normalizedPath == "") {
            // FIXME this should prepend CWD
            path = child;
        } else {
            // FIXME this is stupid
            path = parent.normalizedPath + File.separatorChar + child;
        }

        this.internalInitFileData(Charset.stringToUtf8(path));
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

    public boolean isAbsolute() {
        // TODO windows support
        return this.normalizedPath.charAt(0) == '/';
    }

    public boolean isDirectory() {
        return this.isDirectory;
    }

    public boolean isFile() {
        return this.exists && !this.isDirectory;
    }

    public boolean exists() {
        return this.exists;
    }

    public native String getAbsolutePath();

    public File getAbsoluteFile() {
        return new File(this.getAbsolutePath());
    }

    public native String getCanonicalPath() throws IOException;

    public File getCanonicalFile() throws IOException {
        return new File(this.getCanonicalPath());
    }

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

    public File getParentFile() {
        return new File(this.getParent());
    }

    public String getPath() {
        return this.normalizedPath;
    }

    private native void internalInitFileData(byte[] name);
}
