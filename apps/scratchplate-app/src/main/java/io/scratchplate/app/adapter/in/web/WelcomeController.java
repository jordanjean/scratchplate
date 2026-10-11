package io.scratchplate.app.adapter.in.web;

import io.scratchplate.app.application.port.in.GreetUserUseCase;
import org.springframework.http.MediaType;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationToken;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * Inbound REST adapter exposing the welcome use case.
 */
@RestController
@RequestMapping(path = "/api/v1/welcome", produces = MediaType.APPLICATION_JSON_VALUE)
class WelcomeController {

    private final GreetUserUseCase greetUser;
    private final JwtUserIdentityMapper identityMapper;

    WelcomeController(GreetUserUseCase greetUser, JwtUserIdentityMapper identityMapper) {
        this.greetUser = greetUser;
        this.identityMapper = identityMapper;
    }

    @GetMapping
    @PreAuthorize("hasAnyRole('ADMIN', 'USER')")
    WelcomeResponse welcome(JwtAuthenticationToken authentication) {
        var user = identityMapper.toUserIdentity(authentication);
        return WelcomeResponse.from(greetUser.greet(user));
    }
}

