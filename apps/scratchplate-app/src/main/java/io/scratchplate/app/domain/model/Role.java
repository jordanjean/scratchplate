package io.scratchplate.app.domain.model;

import java.util.Arrays;
import java.util.Locale;
import java.util.Optional;

/**
 * Business roles understood by the application, ordered from the most to the least privileged.
 */
public enum Role {

    ADMIN,
    USER;

    /**
     * Resolves a role from its (case-insensitive) name, e.g. {@code "admin"} or {@code "USER"}.
     */
    public static Optional<Role> fromName(String name) {
        if (name == null || name.isBlank()) {
            return Optional.empty();
        }
        var normalized = name.trim().toUpperCase(Locale.ROOT);
        return Arrays.stream(values())
                .filter(role -> role.name().equals(normalized))
                .findFirst();
    }
}


