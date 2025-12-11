package java.applet;

import java.net.URL;

public interface AppletStub {
    AppletContext getAppletContext();

    boolean isActive();

    URL getDocumentBase();

    String getParameter(String name);

    void appletResize(int width, int height);
}
