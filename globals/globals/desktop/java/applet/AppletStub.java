package java.applet;

public interface AppletStub {
    AppletContext getAppletContext();

    boolean isActive();

    String getParameter(String name);

    void appletResize(int width, int height);
}
