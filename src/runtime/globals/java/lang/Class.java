package java.lang;

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

    public boolean desiredAssertionStatus() {
        // TODO implement
        return false;
    }
}
