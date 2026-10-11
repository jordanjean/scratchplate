package io.scratchplate.app.application.service;

import io.scratchplate.app.application.port.out.WelcomeTemplatePort;
import io.scratchplate.app.domain.exception.MissingRoleException;
import io.scratchplate.app.domain.model.Role;
import io.scratchplate.app.domain.model.UserIdentity;
import org.junit.jupiter.api.Test;

import java.util.Set;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

class GreetUserServiceTest {

    private final WelcomeTemplatePort templates = role -> switch (role) {
        case ADMIN -> "Hello admin {username}";
        case USER -> "Hello user {username}";
    };

    private final GreetUserService service = new GreetUserService(templates);

    @Test
    void greetsAdmin() {
        var message = service.greet(new UserIdentity("alice", Set.of(Role.ADMIN)));

        assertThat(message.role()).isEqualTo(Role.ADMIN);
        assertThat(message.text()).isEqualTo("Hello admin alice");
    }

    @Test
    void greetsUser() {
        var message = service.greet(new UserIdentity("bob", Set.of(Role.USER)));

        assertThat(message.role()).isEqualTo(Role.USER);
        assertThat(message.text()).isEqualTo("Hello user bob");
    }

    @Test
    void mostPrivilegedRoleWins() {
        var message = service.greet(new UserIdentity("carol", Set.of(Role.USER, Role.ADMIN)));

        assertThat(message.role()).isEqualTo(Role.ADMIN);
    }

    @Test
    void rejectsUserWithoutRole() {
        assertThatThrownBy(() -> service.greet(new UserIdentity("dave", Set.of())))
                .isInstanceOf(MissingRoleException.class);
    }
}

