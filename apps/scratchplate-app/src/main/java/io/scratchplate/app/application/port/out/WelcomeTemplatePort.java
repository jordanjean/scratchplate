package io.scratchplate.app.application.port.out;

import io.scratchplate.app.domain.model.Role;

/**
 * Outbound port: provides the welcome message template for a given role.
 * Templates may contain the {@code {username}} placeholder.
 */
public interface WelcomeTemplatePort {

    String templateFor(Role role);
}

