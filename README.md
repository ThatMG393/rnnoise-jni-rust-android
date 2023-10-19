# rnnoise-jni-rust
Simple JNI wrapper for the [nnnoiseless](https://github.com/jneem/nnnoiseless) using [jni-rs](https://github.com/jni-rs/jni-rs).

### Adding dependency to the project
<img alt="version" src="https://img.shields.io/badge/dynamic/xml?label=%20&query=/metadata/versioning/versions/version[not(contains(text(),'%2B'))][last()]&url=https://repo.plasmoverse.com/releases/com/plasmoverse/rnnoise-jni-rust/maven-metadata.xml">

```kotlin
repositories {
    maven("https://repo.plasmoverse.com/releases")
}

dependencies {
    implementation("com.plasmoverse:rnnoise-jni-rust:$version")
}
```

### Usage
Sample code from [DenoiseTest.java](https://github.com/plasmoapp/rnnoise-jni-rust/blob/main/src/test/java/com/plasmoverse/rnnoise/DenoiseTest.java)
```java
// Creates a new RNNoise instance.
Denoise denoise = Denoise.create();

// Processes the samples. Samples should be in 16-bit, 48kHz signed PCM format.
float[] processed = denoise.process(new float[960]);

// Closes the RNNoise instance, releasing allocated resources.
denoise.close();
```
