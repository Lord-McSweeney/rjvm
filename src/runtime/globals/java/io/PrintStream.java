package java.io;

public class PrintStream extends FilterOutputStream {
    public PrintStream(OutputStream out) {
        super(out);
    }

    public void print(boolean b) {
        if (b) {
            this.print("true");
        } else {
            this.print("false");
        }
    }

    public void print(int integer) {
        String stringified = Integer.toString(integer);
        this.print(stringified);
    }

    public void print(Object object) {
        if (object == null) {
            this.print("null");
        } else {
            String stringified = object.toString();
            this.print(stringified);
        }
    }

    public void print(String string) {
        try {
            if (string == null) {
                string = "null";
            }

            byte[] bytes = stringToUtf8(string);

            this.write(bytes);
        } catch (IOException e) { }
    }

    void printNewline() {
        try {
            this.out.write((byte) '\n');
        } catch (IOException e) { }
    }

    public void println(boolean b) {
        this.print(b);
        this.printNewline();
    }

    public void println(int i) {
        this.print(i);
        this.printNewline();
    }

    public void println(Object obj) {
        this.print(obj);
        this.printNewline();
    }

    public void println(String string) {
        this.print(string);
        this.printNewline();
    }

    // TODO implement this with proper Charset decoding
    static native byte[] stringToUtf8(String string);
}
