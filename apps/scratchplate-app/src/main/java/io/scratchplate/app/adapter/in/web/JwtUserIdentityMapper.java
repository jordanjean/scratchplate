package io.scratchplate.app.adapter.in.web;

import io.scratchplate.app.domain.model.Role;
import io.scratchplate.app.domain.model.UserIdentity;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationToken;
import org.springframework.stereotype.Component;

import java.util.Optional;
import java.util.stream.Collectors;

/**
 * Translates a validated JWT authentication into the domain {@link UserIdentity}.
 */
@Component
class JwtUserIdentityMapper {

    static final String ROLE_PREFIX = "ROLE_";
    static final String USERNAME_CLAIM = "preferred_username";

    UserIdentity toUserIdentity(JwtAuthenticationToken authentication) {
        var jwt = authentication.getToken();
        var username = Optional.ofNullable(jwt.getClaimAsString(USERNAME_CLAIM))
                .filter(name -> !name.isBlank())
                .orElse(jwt.getSubject());

        var roles = authentication.getAuthorities().stream()
                .map(GrantedAuthority::getAuthority)
                .filter(authority -> authority != null && authority.startsWith(ROLE_PREFIX))
                .map(authority -> authority.substring(ROLE_PREFIX.length()))
                .map(Role::fromName)
                .flatMap(Optional::stream)
                .collect(Collectors.toSet());

        return new UserIdentity(username, roles);
    }
}

