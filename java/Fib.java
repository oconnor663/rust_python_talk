class Fib
{
    static long fib(long x)
    {
        if (x <= 1) {
            return 1;
        } else {
            return fib(x-1) + fib(x-2);
        }
    }

    public static void main(String []args)
    {
        long x = Long.parseLong(args[0], 10);
        System.out.println(fib(x));
    }
};
