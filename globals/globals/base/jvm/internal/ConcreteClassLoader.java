package jvm.internal;

import java.io.ByteArrayInputStream;
import java.io.InputStream;

class ConcreteClassLoader extends ClassLoader {
    private ConcreteClassLoader() { }

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
}
