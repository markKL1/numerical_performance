package com.keylane;

class JniTest {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native double mul(int n);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("matmullib");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        int n = Integer.parseInt(args[0]);
        assert n > 0: "Number of iterations too low";
        double output = JniTest.mul(n);
        System.out.println(output);
    }
}
