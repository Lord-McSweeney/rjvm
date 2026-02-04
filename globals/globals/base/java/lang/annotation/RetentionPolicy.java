package java.lang.annotation;

public enum RetentionPolicy {
    // Compiler discards annotations
    SOURCE,
    // Compiler puts annotations in class file, but VM discards them
    CLASS,
    // Annotations are always retained
    RUNTIME
}
