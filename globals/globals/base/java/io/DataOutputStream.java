package java.io;

public class DataOutputStream extends FilterOutputStream {
    protected int written;

    public DataOutputStream(OutputStream out) {
        super(out);
    }

    public synchronized void write(int b) throws IOException {
        this.out.write(b);

        this.written += 1;
    }

    public final void writeByte(int b) throws IOException {
        this.out.write(b);

        this.written += 1;
    }

    public final void writeShort(int s) throws IOException {
        int first =  (s >>> 8) & 0xFF;
        int second = (s >>> 0) & 0xFF;

        this.out.write(first);
        this.out.write(second);

        this.written += 2;
    }

    public final void writeInt(int v) throws IOException {
        int first =  (v >>> 24) & 0xFF;
        int second = (v >>> 16) & 0xFF;
        int third =  (v >>> 8) & 0xFF;
        int last =   (v >>> 0) & 0xFF;

        this.out.write(first);
        this.out.write(second);
        this.out.write(third);
        this.out.write(last);

        this.written += 4;
    }
    
    public final void writeUTF(String str) throws IOException {
        // TODO encode properly

        byte[] bytes = str.getBytes();

        this.writeShort(bytes.length);

        for (byte b : bytes) {
            this.writeByte(b);
        }
    }
}
