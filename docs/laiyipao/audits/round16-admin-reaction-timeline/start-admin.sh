#!/bin/bash
cd laiyipao/admin || exit 1
exec npx vite --host 127.0.0.1 --port 5175
