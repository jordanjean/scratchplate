package io.scratchplate.app.domain.model;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.NullAndEmptySource;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.Set;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatIllegalArgumentException;

class UserIdentityTest {

    @ParameterizedTest
    @NullAndEmptySource
    @ValueSource(strings = " ")
    void rejectsBlankUsername(String username) {
        assertThatIllegalArgumentException().isThrownBy(() -> new UserIdentity(username, Set.of()));
    }

    @Test
    void primaryRoleIsMostPrivileged() {
        assertThat(new UserIdentity("u", Set.of(Role.USER, Role.ADMIN)).primaryRole()).contains(Role.ADMIN);
        assertThat(new UserIdentity("u", Set.of(Role.USER)).primaryRole()).contains(Role.USER);
        assertThat(new UserIdentity("u", Set.of()).primaryRole()).isEmpty();
    }

    @Test
    void resolvesRoleNamesCaseInsensitively() {
        assertThat(Role.fromName("admin")).contains(Role.ADMIN);
        assertThat(Role.fromName(" User ")).contains(Role.USER);
        assertThat(Role.fromName("offline_access")).isEmpty();
        assertThat(Role.fromName(null)).isEmpty();
    }
}

