package io.scratchplate.app.adapter.out.config;

import jakarta.validation.constraints.NotBlank;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.validation.annotation.Validated;

/**
 * Externalised welcome message templates ({@code scratchplate.welcome.*}).
 */
@Validated
@ConfigurationProperties(prefix = "scratchplate.welcome")
public record WelcomeTemplateProperties(
        @NotBlank String adminTemplate,
        @NotBlank String userTemplate) {
}

