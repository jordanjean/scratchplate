from __future__ import annotations

import ast
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[2]
PACKAGE_ROOT = PROJECT_ROOT / "scratchplate_app"
FORBIDDEN_PREFIXES = ("fastapi", "jwt", "scratchplate_app.adapter", "scratchplate_app.infrastructure")


def _imports_for(path: Path) -> set[str]:
    tree = ast.parse(path.read_text())
    imports: set[str] = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imports.update(alias.name for alias in node.names)
        elif isinstance(node, ast.ImportFrom) and node.module:
            imports.add(node.module)
    return imports


def test_domain_and_application_layers_do_not_depend_on_web_or_infrastructure() -> None:
    layer_files = list((PACKAGE_ROOT / "domain").glob("*.py"))
    layer_files += list((PACKAGE_ROOT / "application").rglob("*.py"))

    offending = {
        str(path.relative_to(PROJECT_ROOT)): sorted(
            imported
            for imported in _imports_for(path)
            if imported.startswith(FORBIDDEN_PREFIXES)
        )
        for path in layer_files
    }
    offending = {path: imports for path, imports in offending.items() if imports}

    assert offending == {}
