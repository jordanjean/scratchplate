package io.scratchplate.app.infrastructure.config;

import io.scratchplate.app.application.port.in.GreetUserUseCase;
import io.scratchplate.app.application.port.out.WelcomeTemplatePort;
import io.scratchplate.app.application.service.GreetUserService;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

/**
 * Wires framework-free application services into the Spring context.
 */
@Configuration(proxyBeanMethods = false)
class UseCaseConfig {

    @Bean
    GreetUserUseCase greetUserUseCase(WelcomeTemplatePort welcomeTemplatePort) {
        return new GreetUserService(welcomeTemplatePort);
    }
}

