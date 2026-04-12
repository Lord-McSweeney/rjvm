interface IFaceWithDefault {
    void a();

    default void b() {
        System.out.println("IFaceWithDefault.b");
    }
}

interface OverridingIFaceWithDefault extends IFaceWithDefault {
    default void b() {
        System.out.println("OverridingIFaceWithDefault.b");
    }
}

class IFaceExtender1 implements IFaceWithDefault {
    public void a() {
        System.out.println("IFaceExtender1.a");
    }
}

class IFaceExtender2 implements IFaceWithDefault {
    public void a() {
        System.out.println("IFaceExtender2.a");
    }

    public void b() {
        System.out.println("IFaceExtender2.b");
    }
}

class IFaceExtender3 extends IFaceExtender2 {
    public void b() {
        super.b();
        System.out.println("IFaceExtender3.b");
    }
}

class IFaceExtender4 extends IFaceExtender2 { }

class IFaceExtender5 extends IFaceExtender1 { }

class IFaceExtender6 implements OverridingIFaceWithDefault {
    public void a() {
        System.out.println("IFaceExtender6.a");
    }
}

class IFaceExtender7 extends IFaceExtender6 implements IFaceWithDefault {
    public void a() {
        System.out.println("IFaceExtender7.a");
    }
}

class IFaceExtender8 implements IFaceWithDefault, OverridingIFaceWithDefault {
    public void a() {
        System.out.println("IFaceExtender8.a");
    }
}

public class Test {
    public static void main(String[] args) {
        IFaceWithDefault[] arr = new IFaceWithDefault[]{new IFaceExtender1(), new IFaceExtender2(), new IFaceExtender3(), new IFaceExtender4(), new IFaceExtender5(), new IFaceExtender6(), new IFaceExtender7(), new IFaceExtender8()};
        for (IFaceWithDefault iface : arr) {
            System.out.println(iface.getClass());
            iface.a();
            iface.b();
        }
        new IFaceExtender1().a();
        new IFaceExtender1().b();
        new IFaceExtender2().a();
        new IFaceExtender2().b();
        new IFaceExtender3().a();
        new IFaceExtender3().b();
        new IFaceExtender4().a();
        new IFaceExtender4().b();
        new IFaceExtender5().a();
        new IFaceExtender5().b();
        new IFaceExtender6().a();
        new IFaceExtender6().b();
        new IFaceExtender7().a();
        new IFaceExtender7().b();
        new IFaceExtender8().a();
        new IFaceExtender8().b();
    }
}
