package com.plasmoverse.rnnoise;

import org.junit.jupiter.api.Test;

public final class DenoiseTest {

    @Test
    public void process() throws Exception {
        // Creates a new RNNoise instance.
        Denoise denoise = Denoise.create();

        // Processes the samples. Samples should be in 16-bit, 48kHz signed PCM format.
        denoise.process(new float[960]);

        // Closes the RNNoise instance, releasing allocated resources
        denoise.close();
    }
}
