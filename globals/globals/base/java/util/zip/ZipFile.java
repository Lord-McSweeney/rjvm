package java.util.zip;

import java.io.File;

public class ZipFile {
    public ZipFile(File file) {
    }

    public ZipFile(String fileName) {
        this(new File(fileName));
    }
}
