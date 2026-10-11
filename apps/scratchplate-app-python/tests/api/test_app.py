from __future__ import annotations

from datetime import timedelta

from tests.support import make_token


def test_welcome_admin(main_client) -> None:
    token = make_token("id-admin", roles=["admin"], preferred_username="admin")

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 200
    assert response.json() == {
        "message": "Welcome admin! You are signed in as an administrator: you have full access to the platform.",
        "username": "admin",
        "role": "admin",
    }


def test_welcome_user(main_client) -> None:
    token = make_token("id-user", roles=["user"], preferred_username="user")

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 200
    assert response.json()["role"] == "user"
    assert response.json()["message"] == "Welcome user! You are signed in as a regular user."


def test_admin_wins_when_both_roles_are_present(main_client) -> None:
    token = make_token("id-root", roles=["user", "admin"], preferred_username="root")

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 200
    assert response.json()["role"] == "admin"


def test_missing_token_returns_401(main_client) -> None:
    response = main_client.get("/api/v1/welcome")

    assert response.status_code == 401
    assert response.text == ""
    assert response.headers["WWW-Authenticate"] == "Bearer"


def test_bad_signature_returns_401(main_client) -> None:
    token = make_token("id-user", roles=["user"], secret="wrong-secret")

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 401
    assert response.text == ""
    assert response.headers["WWW-Authenticate"] == "Bearer"


def test_expired_token_returns_401(main_client) -> None:
    token = make_token("id-user", roles=["user"], expires_delta=timedelta(minutes=-1))

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 401
    assert response.headers["WWW-Authenticate"] == "Bearer"


def test_wrong_issuer_returns_401(main_client) -> None:
    token = make_token("id-user", issuer="http://wrong-issuer", roles=["user"])

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 401
    assert response.headers["WWW-Authenticate"] == "Bearer"


def test_authenticated_user_without_supported_role_gets_403(main_client) -> None:
    token = make_token("id-guest", roles=["offline_access"], preferred_username="guest")

    response = main_client.get("/api/v1/welcome", headers={"Authorization": f"Bearer {token}"})

    assert response.status_code == 403
    assert response.text == ""


def test_other_paths_require_authentication(main_client) -> None:
    anonymous_response = main_client.get("/anything")
    authenticated_response = main_client.get(
        "/anything",
        headers={"Authorization": f"Bearer {make_token('id-admin', roles=['admin'], preferred_username='admin')}"},
    )

    assert anonymous_response.status_code == 401
    assert anonymous_response.headers["WWW-Authenticate"] == "Bearer"
    assert authenticated_response.status_code == 403


def test_management_health_is_up(management_client) -> None:
    response = management_client.get("/actuator/health/readiness")

    assert response.status_code == 200
    assert response.json() == {"status": "UP"}
