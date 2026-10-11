package io.scratchplate.app.domain.model;

import java.util.Objects;

/**
 * A personalised welcome message addressed to a user acting with a given role.
 */
public record WelcomeMessage(String username, Role role, String text) {

    public static final String USERNAME_PLACEHOLDER = "{username}";

    public WelcomeMessage {
        Objects.requireNonNull(username, "username must not be null");
        Objects.requireNonNull(role, "role must not be null");
        Objects.requireNonNull(text, "text must not be null");
    }

    /**
     * Builds a message by substituting {@value #USERNAME_PLACEHOLDER} in the given template.
     */
    public static WelcomeMessage fromTemplate(String template, UserIdentity user, Role role) {
        Objects.requireNonNull(template, "template must not be null");
        return new WelcomeMessage(user.username(), role, template.replace(USERNAME_PLACEHOLDER, user.username()));
    }
}

