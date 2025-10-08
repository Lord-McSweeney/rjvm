package java.lang.reflect;

public abstract class Executable extends AccessibleObject implements GenericDeclaration, Member {
    // NOTE: THIS FIELD IS ACCESSED FROM NATIVE CODE! FIELD ORDERING MATTERS!
    private int internalId;
}
