package io.scratchplate.app.adapter.out.config;

import io.scratchplate.app.application.port.out.WelcomeTemplatePort;
import io.scratchplate.app.domain.model.Role;
import org.springframework.stereotype.Component;

/**
 * Outbound adapter serving welcome templates from application configuration.
 */
@Component
class ConfigurationWelcomeTemplateAdapter implements WelcomeTemplatePort {

    private final WelcomeTemplateProperties properties;

    ConfigurationWelcomeTemplateAdapter(WelcomeTemplateProperties properties) {
        this.properties = properties;
    }

    @Override
    public String templateFor(Role role) {
        return switch (role) {
            case ADMIN -> properties.adminTemplate();
            case USER -> properties.userTemplate();
        };
    }
}

