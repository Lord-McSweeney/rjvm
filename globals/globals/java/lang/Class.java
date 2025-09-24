package java.lang;

import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;

// NOTE: The native `Class` corresponding to this `Class<T>` is stored in
// `rjvm_core`'s `Context` and can be retrieved with `Context::class_for_java_class`
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

    public String toString() {
        StringBuilder result = new StringBuilder();

        if (this.isInterface()) {
            result.append("interface ");
        } else {
            result.append("class ");
        }

        result.append(this.getName());

        return result.toString();
    }
}
