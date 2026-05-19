package java.awt;

public abstract class Component {
    protected Component() { }

    public Toolkit getToolkit() {
        return null;
    }

    public void paint(Graphics g) {
        // Component.paint does nothing
    }

    public void update(Graphics g) {
        this.paint(g);
    }

    // Legacy mouse handlers

    public boolean mouseDown(Event evt, int x, int y) {
        return false;
    }

    public boolean mouseUp(Event evt, int x, int y) {
        return false;
    }
}
