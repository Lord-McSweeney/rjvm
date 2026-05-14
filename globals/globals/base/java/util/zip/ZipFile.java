package java.util.zip;

import java.io.Closeable;
import java.io.File;
import java.io.InputStream;
import java.io.IOException;
import java.util.Enumeration;

public class ZipFile implements Closeable {
    private File file;

    public ZipFile(File file) {
        this.file = file;
    }

    public ZipFile(String fileName) {
        this(new File(fileName));
    }

    public ZipEntry getEntry(String name) {
        return null;
    }

    public Enumeration<? extends ZipEntry> entries() {
        return null;
    }

    public InputStream getInputStream(ZipEntry entry) throws IOException {
        return null;
    }

    public void close() {
    }
}
