package io.scratchplate.app.adapter.in.web;

import io.scratchplate.app.domain.model.WelcomeMessage;

import java.util.Locale;

/**
 * REST representation of a welcome message.
 */
public record WelcomeResponse(String message, String username, String role) {

    static WelcomeResponse from(WelcomeMessage welcome) {
        return new WelcomeResponse(welcome.text(), welcome.username(), welcome.role().name().toLowerCase(Locale.ROOT));
    }
}



