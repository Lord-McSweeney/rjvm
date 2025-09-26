public class Test {
    public static void main(String[] args) {
        System.out.println((int) (Math.atan2(3, 0) * 1000));
        System.out.println((int) (Math.atan2(338.72823, 32878.213) * 1000));
        System.out.println((int) (Math.atan2(3.72823, 78.213) * 1000));
        System.out.println((int) (Math.atan2(-338.72823, -32878.213) * 1000));
        System.out.println((int) (Math.atan2(-3.72823, -78.213) * 1000));
        System.out.println((int) (Math.atan2(-338.72823, 32878.213) * 1000));
        System.out.println((int) (Math.atan2(-3.72823, 78.213) * 1000));
        System.out.println((int) (Math.atan2(338.72823, -32878.213) * 1000));
        System.out.println((int) (Math.atan2(3.72823, -78.213) * 1000));
        System.out.println((int) (Math.atan2(0, 0) * 1000));
        System.out.println((int) (Math.atan2(0, -2) * 1000));

        System.out.println((int) Math.pow(3, 0));
        System.out.println((int) Math.pow(338.72823, 32878.213));

        System.out.println((int) Math.sqrt(3.0));
        System.out.println((int) Math.sqrt(1789));
        System.out.println((int) Math.sqrt(400));
        System.out.println((int) Math.sqrt(234774832.278));
        System.out.println((int) Math.sqrt(0.778));

        System.out.println((int) Math.log(2));
        System.out.println((int) Math.log(3.0));
        System.out.println((int) Math.log(1789));
        System.out.println((int) Math.log(400));
        System.out.println((int) Math.log(234774832.278));
        System.out.println((int) Math.log(0.778));
    }
}
