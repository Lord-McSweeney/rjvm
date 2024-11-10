package java.lang;

import java.io.InputStream;

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

    public InputStream getResourceAsStream(String name) {
        // TODO implement
        return null;
    }

    public boolean desiredAssertionStatus() {
        // TODO implement
        return false;
    }
}
