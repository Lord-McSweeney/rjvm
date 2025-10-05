package java.io;

public class FileWriter extends OutputStreamWriter {
    public FileWriter(File file) throws FileNotFoundException {
        super(new FileOutputStream(file));
    }

    public FileWriter(String fileName) throws FileNotFoundException {
        super(new FileOutputStream(fileName));
    }
}
