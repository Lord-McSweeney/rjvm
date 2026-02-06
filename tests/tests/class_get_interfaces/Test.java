import java.util.HashMap;

interface IF { }

interface IE extends IF { }

interface ID { }

interface IC { }

interface IB { }

interface IA extends IB { }

class B implements IA, ID { }

class A extends B implements IC, IE {
}

public class Test {
    public static void main(String[] args) throws Exception {
        for (Class iface : A.class.getInterfaces()) {
            System.out.println(iface);
        }
    }
}
