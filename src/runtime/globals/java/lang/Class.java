package java.lang;

import java.io.InputStream;
import java.io.IOException;

class DataStream extends InputStream {
    byte[] data;
    int position;

    public DataStream(byte[] data) {
        this.data = data;
        this.position = 0;
    }

    public int available() throws IOException {
        return this.data.length - this.position;
    }

    public int read() throws IOException {
        this.position ++;
        if (this.position == this.data.length) {
            // Make sure to revert position so a call to available()
            // doesn't return a negative number
            this.position --;

            return -1;
        } else {
            return this.data[this.position - 1];
        }
    }
}

public final class Class<T> {
    private Class() { }

    private String cachedName;
    public String getName() {
        if (this.cachedName == null) {
            String name = this.getNameNative();
            this.cachedName = name;
        }

        return this.cachedName;
    }

    private native String getNameNative();

    public native boolean isInterface();

    public InputStream getResourceAsStream(String resourceName) {
        if (resourceName == null) {
            throw new NullPointerException();
        }

        byte[] resourceData = this.getResourceData(resourceName);

        if (resourceData != null) {
            return new DataStream(resourceData);
        } else {
            return null;
        }
    }

    private native byte[] getResourceData(String resourceName);

    public boolean desiredAssertionStatus() {
        // TODO implement
        return false;
    }
}
