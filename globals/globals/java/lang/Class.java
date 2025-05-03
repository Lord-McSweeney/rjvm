package java.lang;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;

public final class Class<T> {
    private int classId;

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
            return new ByteArrayInputStream(resourceData);
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
