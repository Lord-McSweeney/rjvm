package java.util.zip;

import java.io.Closeable;
import java.io.File;
import java.io.InputStream;
import java.io.IOException;

public class ZipFile implements Closeable {
    private File file;

    public ZipFile(File file) {
        this.file = file;
    }

    public ZipFile(String fileName) {
        this(new File(fileName));
    }

    public InputStream getInputStream(ZipEntry entry) throws IOException {
        return null;
    }

    public void close() {
    }
}
