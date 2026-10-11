package io.scratchplate.app.domain.exception;

/**
 * Raised when a user does not hold any role the application knows how to greet.
 */
public class MissingRoleException extends RuntimeException {

    public MissingRoleException(String username) {
        super("User '%s' does not hold any supported role".formatted(username));
    }
}

