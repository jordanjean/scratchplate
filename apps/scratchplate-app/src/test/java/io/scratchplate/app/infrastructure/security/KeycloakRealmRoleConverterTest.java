package io.scratchplate.app.infrastructure.security;

import org.junit.jupiter.api.Test;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.security.oauth2.jwt.Jwt;

import java.util.List;
import java.util.Map;

import static org.assertj.core.api.Assertions.assertThat;

class KeycloakRealmRoleConverterTest {

    private final KeycloakRealmRoleConverter converter = new KeycloakRealmRoleConverter();

    @Test
    void mapsRealmRolesToPrefixedAuthorities() {
        var jwt = jwtWithClaim("realm_access", Map.of("roles", List.of("admin", "offline_access")));

        assertThat(converter.convert(jwt))
                .extracting(GrantedAuthority::getAuthority)
                .containsExactlyInAnyOrder("ROLE_ADMIN", "ROLE_OFFLINE_ACCESS");
    }

    @Test
    void returnsNoAuthoritiesWithoutRealmAccess() {
        assertThat(converter.convert(jwtWithClaim("scope", "openid"))).isEmpty();
    }

    private static Jwt jwtWithClaim(String name, Object value) {
        return Jwt.withTokenValue("token").header("alg", "RS256").subject("sub").claim(name, value).build();
    }
}

