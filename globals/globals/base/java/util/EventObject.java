package java.util;

public class EventObject {
    protected Object source;

    public EventObject(Object source) {
        if (source == null) {
            throw new IllegalArgumentException();
        }
        this.source = source;
    }

    public Object getSource() {
        return this.source;
    }
}
