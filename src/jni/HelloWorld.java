class HelloWorld {

    private static native String hello(String name);

    private static native void callback(int n, HelloWorld callback, String methodName);

    private static native long counterNew(HelloWorld callback, String methodName);
    private static native void counterIncrement(long counter_ptr);
    private static native void counterDestroy(long counter_ptr);

    private static native void asyncComputation(HelloWorld callback);

    static {
        System.loadLibrary("jni");
    }

    public static void main(String[] args) {
        String output = HelloWorld.hello("josh");
        System.out.println(output);

        HelloWorld.callback(6, new HelloWorld(), "method");

        long counter_ptr = counterNew(new HelloWorld(), "counterCallback");

        for (int i = 0; i < 5; i++) {
            counterIncrement(counter_ptr);
        }

        counterDestroy(counter_ptr);

        System.out.println("Start Thread Id : " + Thread.currentThread().getId());
        asyncComputation(new HelloWorld());
        System.out.println("End Thread Id : " + Thread.currentThread().getId());
    }

    public void method(int res) {
        System.out.println("callback: " + res);
    }

    public void counterCallback(int count) {
        System.out.println("counterCallback: count = " + count);
    }

    public void asyncCallback(int progress) {
        System.out.println("asyncCallback: progress = " + progress);
    }
}