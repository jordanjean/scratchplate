package io.scratchplate.app.adapter.in.web;

import io.scratchplate.app.domain.exception.MissingRoleException;
import org.springframework.http.HttpStatus;
import org.springframework.http.ProblemDetail;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

/**
 * Maps domain exceptions to RFC 9457 problem details.
 */
@RestControllerAdvice
class WebExceptionHandler {

    @ExceptionHandler(MissingRoleException.class)
    ProblemDetail handleMissingRole(MissingRoleException exception) {
        var problem = ProblemDetail.forStatusAndDetail(HttpStatus.FORBIDDEN, exception.getMessage());
        problem.setTitle("Missing role");
        return problem;
    }
}

