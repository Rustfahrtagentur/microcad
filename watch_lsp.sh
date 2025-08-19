#!/bin/sh

watch -n 0.1 'ps aux | grep -v grep | grep -q "microcad-lsp -l " && echo "microcad-lsp running" || echo "microcad-lsp NOT running"'
