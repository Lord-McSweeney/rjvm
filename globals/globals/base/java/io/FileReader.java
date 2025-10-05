package java.io;

public class FileReader extends InputStreamReader {
    public FileReader(File file) throws FileNotFoundException {
        super(new FileInputStream(file));
    }

    public FileReader(String fileName) throws FileNotFoundException {
        super(new FileInputStream(fileName));
    }
}
