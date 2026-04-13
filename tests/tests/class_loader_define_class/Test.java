import java.io.*;

class LoadableClass {
    
}

class LoaderExtender extends ClassLoader {
    public LoaderExtender() {
        super(null);
    }
    
    public void doIt(byte[] data) {
        super.defineClass("LoadableClass", data, 0, data.length);
    }
    
    public void doIt2(byte[] data) {
        super.defineClass("LoadableClass2", data, 0, data.length);
    }
}

public class Test {
    public static void main(String[] args) throws Exception {
        FileInputStream stream = new FileInputStream("LoadableClass.class");
        byte[] data = new byte[stream.available()];
        stream.read(data);
        System.out.println(data.length);
        LoaderExtender loader = new LoaderExtender();
        System.out.println(ClassLoader.getSystemClassLoader().loadClass("LoadableClass"));
        try {
            System.out.println(loader.loadClass("LoadableClass"));
        } catch(Throwable e) {
            System.out.println(e.getClass());
        }
        loader.doIt(data);
        try {
            System.out.println(loader.loadClass("LoadableClass"));
        } catch(Throwable e) {
            System.out.println(e.getClass());
        }
        try {
            loader.doIt(data);
        } catch(Throwable e) {
            System.out.println(e.getClass());
        }
        try {
            loader.doIt2(data);
        } catch(Throwable e) {
            System.out.println(e.getClass());
        }
        try {
            System.out.println(loader.loadClass("LoadableClass"));
        } catch(Throwable e) {
            System.out.println(e.getClass());
        }
    }
}
