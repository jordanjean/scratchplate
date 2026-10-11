package io.scratchplate.app.application.service;

import io.scratchplate.app.application.port.in.GreetUserUseCase;
import io.scratchplate.app.application.port.out.WelcomeTemplatePort;
import io.scratchplate.app.domain.exception.MissingRoleException;
import io.scratchplate.app.domain.model.UserIdentity;
import io.scratchplate.app.domain.model.WelcomeMessage;

import java.util.Objects;

/**
 * Use case implementation. Framework-free: wired as a bean by the infrastructure layer.
 */
public class GreetUserService implements GreetUserUseCase {

    private final WelcomeTemplatePort welcomeTemplates;

    public GreetUserService(WelcomeTemplatePort welcomeTemplates) {
        this.welcomeTemplates = Objects.requireNonNull(welcomeTemplates);
    }

    @Override
    public WelcomeMessage greet(UserIdentity user) {
        Objects.requireNonNull(user, "user must not be null");
        var role = user.primaryRole().orElseThrow(() -> new MissingRoleException(user.username()));
        return WelcomeMessage.fromTemplate(welcomeTemplates.templateFor(role), user, role);
    }
}

