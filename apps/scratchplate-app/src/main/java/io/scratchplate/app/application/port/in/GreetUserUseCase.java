package io.scratchplate.app.application.port.in;

import io.scratchplate.app.domain.model.UserIdentity;
import io.scratchplate.app.domain.model.WelcomeMessage;

/**
 * Inbound port: greet an authenticated user according to their most privileged role.
 */
public interface GreetUserUseCase {

    WelcomeMessage greet(UserIdentity user);
}

