package java.applet;

import java.awt.Image;
import java.awt.Panel;
import java.net.MalformedURLException;
import java.net.URL;

public class Applet extends Panel {
    private AppletStub stub;

    public Applet() { }

    public final void setStub(AppletStub stub) {
        this.stub = stub;
    }

    public AppletContext getAppletContext() {
        return this.stub.getAppletContext();
    }

    // Methods that use the stub

    public URL getDocumentBase() {
        return this.stub.getDocumentBase();
    }

    public String getParameter(String name) {
        return this.stub.getParameter(name);
    }

    public boolean isActive() {
        if (this.stub != null) {
            return this.stub.isActive();
        } else {
            return false;
        }
    }

    // Methods that use the context

    public Image getImage(URL url) {
        return this.stub.getAppletContext().getImage(url);
    }

    public Image getImage(URL url, String name) {
        try {
            return this.getImage(new URL(url, name));
        } catch (MalformedURLException e) {
            return null;
        }
    }

    // Methods to be overriden

    public void init() {
    }

    public void start() {
    }

    public void stop() {
    }

    public void destroy() {
    }
}
