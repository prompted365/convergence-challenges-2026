"""Path-local imports for legacy challenge harnesses.

Several harnesses intentionally use the same filenames, such as
`scenarios.py`, `constraint_runner.py`, and `test_scenarios.py`. Loading them as
top-level modules makes combined pytest runs bleed one challenge's modules into
another. This helper loads sibling harness modules under a challenge-specific
name so full-suite runs can share one Python process.
"""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from types import ModuleType


def load_local_module(anchor_file: str, module_name: str) -> ModuleType:
    """Load `module_name.py` from the same directory as `anchor_file`."""
    anchor = Path(anchor_file).resolve()
    harness_dir = anchor.parent
    challenge_slug = harness_dir.parent.name.replace("-", "_")
    unique_name = f"challenge_{challenge_slug}_{module_name}"
    if unique_name in sys.modules:
        return sys.modules[unique_name]

    module_path = harness_dir / f"{module_name}.py"
    spec = importlib.util.spec_from_file_location(unique_name, module_path)
    if spec is None or spec.loader is None:
        raise ImportError(f"Could not load {module_name} from {module_path}")

    module = importlib.util.module_from_spec(spec)
    sys.modules[unique_name] = module
    spec.loader.exec_module(module)
    return module
