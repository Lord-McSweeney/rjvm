package java.awt;

import java.awt.image.ImageObserver;

public abstract class Image {
    public abstract Graphics getGraphics();

    public abstract int getWidth(ImageObserver observer);

    public abstract int getHeight(ImageObserver observer);

    public void flush() {
        // TODO do we need to do anything?
    }
}
