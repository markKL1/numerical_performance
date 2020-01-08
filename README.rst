
Try to achieve fast matrix multiplication, usually (n^3), by

* Compiling for native architecture, using link-time optimization.
* Removing bound checks.
* Reducing checks by manually unrolling loops.
* Creating good memory alignment by transposing one matrix (for fast cache).
* Using SIMD (which needs the above) on the inner loop.
* Using multithreading on the outer loop.
* There is a faster-than-O(3) algorithm, but I didn't implement that.

There's also some code to see if all this still works through JNI.

Some output::

    >> C
    optimized build
    Hello world from C!
    Sum of product elements = 936451904.627503.
    Time taken = 2.613000 second.

    >> Rust naive
    Hello world from Rust!
    Sum of product elements = 936451904.627516.
    Time taken = 3.967146 second.

    >> Rust opt
    Hello world from Rust!
    Sum of product elements = 936451904.627516.
    Time taken = 0.439449 second.

    >> Rust simd
    Hello world from Rust!
    Sum of product elements = 936451904.627503.
    Time taken = 0.575075 second.

    >> Rust parallel
    Hello world from Rust!
    Sum of product elements = 936451904.627504.
    Time taken = 0.116171 second.

    >> Java (parallel)
    Hello world from Rust (JNI)!
    Sum of product elements = 936451904.627504.
    Time taken = 0.104442 second.

I tried to measure JVM overhead, but the difference in Rust and JVM overhead seems to depend on the data size. At tiny matrices, Java takes 75 ms extra, whereas at 1000 items it takes 81 ms, but at large matrices no difference is noticeable.
