package java.io;

public class PrintStream extends FilterOutputStream {
    public OutputStream out;

    public PrintStream(OutputStream out) {
        super(out);

        this.out = out;
    }

    public void print(String string) {
        try {
            if (string == null) {
                string = "null";
            }

            byte[] bytes = stringToUtf8(string);

            this.out.write(bytes);
        } catch (IOException e) { }
    }

    public void println(String string) {
        this.print(string);
        this.print("\n");
    }

    // TODO implement this with proper Charset decoding
    private static native byte[] stringToUtf8(String string);
}
