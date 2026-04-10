package jvm.internal;

import java.io.ByteArrayInputStream;
import java.io.InputStream;

public class ClassLoaderUtils {
    public static ClassLoader createPlatformLoader() {
        ClassLoader loader = new PlatformClassLoader();
        ClassLoaderUtils.makePlatformLoader(loader);
        return loader;
    }
    public static native void makePlatformLoader(ClassLoader loader);

    public static ClassLoader createSystemLoader(ClassLoader platformLoader) {
        ClassLoader loader = new SystemClassLoader(platformLoader);
        ClassLoaderUtils.makeSystemLoader(platformLoader, loader);
        return loader;
    }
    public static native void makeSystemLoader(ClassLoader platformLoader, ClassLoader loader);
}

final class PlatformClassLoader extends ClassLoader {
    PlatformClassLoader() {
        super(null);
    }

    // Is this supposed to do anything?
}

final class SystemClassLoader extends ClassLoader {
    SystemClassLoader(ClassLoader platformLoader) {
        super(platformLoader);
    }

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

    protected Class<?> findClass(String name) throws ClassNotFoundException {
        if (name == null) {
            throw new NullPointerException();
        }

        // Class names passed to `findClass` must have dots, not slashes
        if (name.indexOf('/') != -1) {
            throw new ClassNotFoundException(name);
        }
        // Array classes are never resolved
        if (name.length() != 0 && name.charAt(0) == '[') {
            throw new ClassNotFoundException(name);
        }

        Class<?> result = SystemClassLoader.loadSystemClassNative(name);
        if (result == null) {
            throw new ClassNotFoundException(name);
        } else {
            return result;
        }
    }

    private static native Class<?> loadSystemClassNative(String name);
}
