class MissingRoleError(RuntimeError):
    def __init__(self, username: str) -> None:
        super().__init__(f"User '{username}' does not hold any supported role")
