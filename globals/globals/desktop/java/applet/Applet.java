package java.applet;

import java.awt.Panel;

public class Applet extends Panel {
    private AppletStub stub;

    public Applet() { }

    public final void setStub(AppletStub stub) {
        this.stub = stub;
    }

    // Methods that use the stub

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
