package io.scratchplate.app.adapter.in.web;

import io.scratchplate.app.infrastructure.security.KeycloakRealmRoleConverter;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.boot.webmvc.test.autoconfigure.AutoConfigureMockMvc;
import org.springframework.test.web.servlet.MockMvc;
import org.springframework.test.web.servlet.request.RequestPostProcessor;

import java.util.List;
import java.util.Map;

import static org.springframework.security.test.web.servlet.request.SecurityMockMvcRequestPostProcessors.jwt;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.jsonPath;
import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.status;

@SpringBootTest
@AutoConfigureMockMvc
class WelcomeApiIntegrationTest {

    private static final String WELCOME = "/api/v1/welcome";

    @Autowired
    private MockMvc mockMvc;

    @Test
    void welcomesAdmin() throws Exception {
        mockMvc.perform(get(WELCOME).with(keycloakToken("admin", "admin", "default-roles-scratchplate")))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.username").value("admin"))
                .andExpect(jsonPath("$.role").value("admin"))
                .andExpect(jsonPath("$.message").value(
                        "Welcome admin! You are signed in as an administrator: you have full access to the platform."));
    }

    @Test
    void welcomesUser() throws Exception {
        mockMvc.perform(get(WELCOME).with(keycloakToken("user", "user")))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.username").value("user"))
                .andExpect(jsonPath("$.role").value("user"))
                .andExpect(jsonPath("$.message").value("Welcome user! You are signed in as a regular user."));
    }

    @Test
    void adminRoleTakesPrecedence() throws Exception {
        mockMvc.perform(get(WELCOME).with(keycloakToken("root", "user", "admin")))
                .andExpect(status().isOk())
                .andExpect(jsonPath("$.role").value("admin"));
    }

    @Test
    void forbidsTokenWithoutSupportedRole() throws Exception {
        mockMvc.perform(get(WELCOME).with(keycloakToken("guest", "offline_access")))
                .andExpect(status().isForbidden());
    }

    @Test
    void rejectsMissingToken() throws Exception {
        mockMvc.perform(get(WELCOME)).andExpect(status().isUnauthorized());
    }

    @Test
    void exposesHealthWithoutToken() throws Exception {
        mockMvc.perform(get("/actuator/health/readiness")).andExpect(status().isOk());
    }

    @Test
    void deniesUnknownPaths() throws Exception {
        mockMvc.perform(get("/anything").with(keycloakToken("admin", "admin")))
                .andExpect(status().isForbidden());
    }

    private static RequestPostProcessor keycloakToken(String username, String... realmRoles) {
        return jwt()
                .jwt(token -> token
                        .subject("id-" + username)
                        .claim("preferred_username", username)
                        .claim("realm_access", Map.of("roles", List.of(realmRoles))))
                .authorities(new KeycloakRealmRoleConverter());
    }
}


