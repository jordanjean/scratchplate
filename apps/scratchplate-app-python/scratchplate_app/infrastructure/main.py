from __future__ import annotations

import asyncio

from scratchplate_app.infrastructure.app_factory import run_servers


def main() -> None:
    asyncio.run(run_servers())


if __name__ == "__main__":
    main()
