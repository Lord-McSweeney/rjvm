package java.lang;

import java.lang.reflect.Constructor;
import java.lang.reflect.Executable;

public final class StackTraceElement {
    // NOTE These fields are set from native code, field ordering matters!
    private String declaringClass;
    private String methodName;
    private String fileName;
    private int lineNumber;
    private boolean isNativeMethod;

    public StackTraceElement(String declaringClass, String methodName, String fileName, int lineNumber) {
        if (declaringClass == null || methodName == null) {
            throw new NullPointerException();
        }

        this.declaringClass = declaringClass;
        this.methodName = methodName;
        this.fileName = fileName;
        this.lineNumber = lineNumber;

        // TODO
        this.isNativeMethod = false;
    }

    public String getClassName() {
        return this.declaringClass;
    }

    public String getMethodName() {
        return this.methodName;
    }

    public String getFileName() {
        return this.fileName;
    }

    public int getLineNumber() {
        return this.lineNumber;
    }

    public boolean isNativeMethod() {
        return this.isNativeMethod;
    }

    public String toString() {
        return this.declaringClass + '.' + this.methodName + "()";
    }
}
