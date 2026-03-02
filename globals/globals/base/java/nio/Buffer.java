package java.nio;

public abstract class Buffer {
    int position;
    int limit;

    public final int position() {
        return this.position;
    }

    public final Buffer flip() {
        this.limit = this.position;
        this.position = 0;
        return this;
    }

    int checkGetNextPosition() {
        if (this.position >= this.limit) {
            throw new BufferOverflowException();
        } else {
            int originalValue = this.position;
            this.position += 1;
            return originalValue;
        }
    }
}
