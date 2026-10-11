package io.scratchplate.app.domain.model;

import java.util.Comparator;
import java.util.EnumSet;
import java.util.Objects;
import java.util.Optional;
import java.util.Set;

/**
 * The authenticated caller, as seen by the domain (independent of any token format).
 */
public record UserIdentity(String username, Set<Role> roles) {

    public UserIdentity {
        if (username == null || username.isBlank()) {
            throw new IllegalArgumentException("username must not be blank");
        }
        Objects.requireNonNull(roles, "roles must not be null");
        roles = roles.isEmpty() ? Set.of() : Set.copyOf(EnumSet.copyOf(roles));
    }

    /**
     * @return the most privileged role held by the user, if any.
     */
    public Optional<Role> primaryRole() {
        return roles.stream().min(Comparator.naturalOrder());
    }

    public boolean hasRole(Role role) {
        return roles.contains(role);
    }
}

