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
        int fourth = (v >>> 0) & 0xFF;

        this.out.write(first);
        this.out.write(second);
        this.out.write(third);
        this.out.write(fourth);

        this.written += 4;
    }

    public final void writeLong(long v) throws IOException {
        int first =   (int) ((v >>> 56) & 0xFF);
        int second =  (int) ((v >>> 48) & 0xFF);
        int third =   (int) ((v >>> 40) & 0xFF);
        int fourth =  (int) ((v >>> 32) & 0xFF);
        int fifth =   (int) ((v >>> 24) & 0xFF);
        int sixth =   (int) ((v >>> 16) & 0xFF);
        int seventh = (int) ((v >>> 8) & 0xFF);
        int eighth =  (int) ((v >>> 0) & 0xFF);

        this.out.write(first);
        this.out.write(second);
        this.out.write(third);
        this.out.write(fourth);
        this.out.write(fifth);
        this.out.write(sixth);
        this.out.write(seventh);
        this.out.write(eighth);

        this.written += 8;
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
