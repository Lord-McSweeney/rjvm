package java.util;

import java.io.InputStream;
import java.io.IOException;

// TODO implement this properly
public class Scanner {
    private InputStream stream;

    public Scanner(InputStream stream) {
        this.stream = stream;
    }

    public String nextLine() {
        char[] data = new char[1];
        int position = 0;
        while (true) {
            char next;
            try {
                int nextRaw = this.stream.read();
                if (nextRaw < 0) {
                    throw new NoSuchElementException();
                } else {
                    next = (char) nextRaw;
                }
            } catch(IOException e) {
                throw new NoSuchElementException();
            }

            // FIXME: `\r`
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
}
