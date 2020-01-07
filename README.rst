
Try to achieve fast matrix multiplication, usually (n^3), by

* Compiling for native architecture, using link-time optimization.
* Removing bound checks.
* Reducing checks by manually unrolling loops.
* Creating good memory alignment by transposing one matrix (for fast cache).
* Using SIMD (which needs the above) on the inner loop.
* Using multithreading on the outer loop.
* There is a faster-than-O(3) algorithm, but I didn't implement that.

There's also some code to see if all this still works through JNI.

Some random things:

* There are steps to show which takes how long.
* Look at JVM startup time.

