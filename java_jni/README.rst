
Rust jni: https://docs.rs/jni/0.14.0/jni/

To run::

    mvn package -DskipTests
    java -jar .\target\JniRustTest-development.jar

To regenerate headers::

    javac -h . src\com\keylane\JniTest.java

