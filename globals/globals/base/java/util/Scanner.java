package java.util;

import java.io.InputStream;
import java.io.IOException;

// TODO implement this properly
public class Scanner {
    private InputStream stream;

    private static int BUFFER_SIZE = 8192;
    private char[] buffer;
    private int bufferPos;
    private int bufferSize;

    public Scanner(InputStream stream) {
        this.stream = stream;
        this.buffer = new char[BUFFER_SIZE];
        this.bufferPos = 0;
        this.bufferSize = 0;
    }

    private void tryFillBuffer() {
        byte[] array = new byte[BUFFER_SIZE - this.bufferPos];
        try {
            int numRead = this.stream.read(array);

            // TODO implement proper decoding
            for (int i = 0; i < numRead; i ++) {
                this.buffer[i + this.bufferPos] = (char) array[i];
            }

            this.bufferSize += numRead;
        } catch(IOException e) {
            throw new NoSuchElementException();
        }
    }

    private char nextChar() {
        if (this.bufferPos == this.bufferSize) {
            // Reset and refill the buffer
            this.bufferPos = 0;
            this.bufferSize = 0;
            this.tryFillBuffer();
        }

        return this.buffer[this.bufferPos ++];
    }

    private void backtrack() {
        this.bufferPos -= 1;
    }

    public String nextLine() {
        char[] data = new char[1];
        int position = 0;
        while (true) {
            char next = this.nextChar();

            // FIXME: `\r`
            // TODO custom delimeters
            if (next == '\n') {
                return new String(data);
            }

            if (data.length == position) {
                char[] newData = new char[data.length * 2];
                System.arraycopy(data, 0, newData, 0, data.length);
                data = newData;
            }
            data[position] = next;
            position += 1;
        }
    }

    public String next() {
        boolean skippingWhitespace = true;
        char[] data = new char[1];
        int position = 0;
        while (true) {
            char next = this.nextChar();

            // TODO custom delimiters
            if (skippingWhitespace) {
                if (next != ' ' && next != '\n' && next != '\r' && next != '\t') {
                    skippingWhitespace = false;
                }
            }

            if (!skippingWhitespace) {
                if (next == ' ' || next == '\n' || next == '\r' || next == '\t') {
                    this.backtrack();
                    return new String(data);
                }
                if (data.length == position) {
                    char[] newData = new char[data.length * 2];
                    System.arraycopy(data, 0, newData, 0, data.length);
                    data = newData;
                }
                data[position] = next;
                position += 1;
            }
        }
    }

    public int nextInt() {
        try {
            return Integer.parseInt(this.next());
        } catch(NumberFormatException e) {
            // TODO: Somehow we have to backtrack to the start of the token...
            throw new InputMismatchException();
        }
    }
}
