# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""确保 llm-lab 根目录在 import 路径上，使 ``import minigrad`` 可用。"""

import os
import sys

sys.path.insert(0, os.path.dirname(__file__))
