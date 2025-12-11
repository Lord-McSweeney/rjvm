package java.applet;

public interface AppletStub {
    boolean isActive();

    String getParameter(String name);

    void appletResize(int width, int height);
}
