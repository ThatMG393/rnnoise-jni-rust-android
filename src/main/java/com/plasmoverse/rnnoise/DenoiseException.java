package com.plasmoverse.rnnoise;

/**
 * Exception indicates issues related to RNNoise.
 */
public class DenoiseException extends Exception {

    public DenoiseException() {
        super();
    }

    public DenoiseException(String message) {
        super(message);
    }

    public DenoiseException(String message, Throwable cause) {
        super(message, cause);
    }

    public DenoiseException(Throwable cause) {
        super(cause);
    }
}
