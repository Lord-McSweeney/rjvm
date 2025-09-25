package javax.swing;

import java.awt.Component;
import rjvm.internal.Todo;

public class JOptionPane extends JComponent {
    public static void showMessageDialog(Component parentComponent, Object message, String title, int messageType) {
        Todo.warnNotImpl("JOptionPane.showMessageDialog");
    }
}
