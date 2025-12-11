package java.applet;

import java.awt.Image;
import java.net.URL;

public interface AppletContext {
    Image getImage(URL url);
}
