package java.lang;

import java.lang.reflect.Constructor;
import java.lang.reflect.Executable;

public final class StackTraceElement {
    private String declaringClass;
    private String methodName;
    private String fileName;
    private int lineNumber;
    private boolean isNativeMethod;

    // NOTE: THIS METHOD IS CALLED FROM NATIVE CODE! METHOD ORDERING MATTERS!
    private static StackTraceElement create(Executable exec) {
        // TODO implement file name and line number
        if (exec instanceof Constructor) {
            return new StackTraceElement(
                exec.getDeclaringClass().getName(),
                // `Constructor.getName` always returns the class name instead
                // of "<init>", so we need a special case for it
                "<init>",
                null,
                0
            );
        } else {
            return new StackTraceElement(
                exec.getDeclaringClass().getName(),
                exec.getName(),
                null,
                0
            );
        }
    }

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
