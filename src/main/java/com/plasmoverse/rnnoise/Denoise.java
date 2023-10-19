package com.plasmoverse.rnnoise;

import java.io.IOException;

public final class Denoise {

    /**
     * Creates a new native RNNoise instance.
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native libraries fail to load.
     * @throws DenoiseException If the RNNoise fail to initialize.
     * @return An instance of the RNNoise.
     */
    public static Denoise create() throws IOException, DenoiseException {
        DenoiseLibrary.load();

        long pointer = createNative();

        return new Denoise(pointer);
    }

    private static native long createNative();


    private final long pointer;

    private Denoise(long pointer) {
        this.pointer = pointer;
    }

    /**
     * Processes the given audio samples with RNNoise.
     *
     * <p>
     *     Samples should be in 16-bit, 48kHz signed PCM format.
     * </p>
     *
     * @param samples The audio samples to process.
     * @return A processed array of audio samples represented as floats.
     * @throws DenoiseException If there's an error during the decoding process.
     */
    public float[] process(float[] samples) throws DenoiseException {
        if (!isOpen()) throw new DenoiseException("Denoise is closed");

        return processNative(samples);
    }

    /**
     * Closes the RNNoise instance, releasing any allocated resources.
     */
    public void close() {
        if (!isOpen()) return;

        closeNative();
    }

    /**
     * Checks if the RNNoise instance is currently open and ready for processing.
     *
     * @return {@code true} if the instance is open, {@code false} otherwise.
     */
    public boolean isOpen() {
        return pointer > 0;
    }

    private native float[] processNative(float[] samples);

    private native void closeNative();
}
