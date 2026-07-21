#!/usr/bin/env bash

set -euo pipefail

PROJECT="hypr-download-sorter"

BIN_FILE="${HOME}/.local/bin/${PROJECT}"
CONFIG_DIR="${HOME}/.config/${PROJECT}"

SERVICE_DIR="${HOME}/.config/systemd/user"
SERVICE_FILE="${SERVICE_DIR}/${PROJECT}.service"

PURGE=false

if [[ "${1:-}" == "--purge" ]]; then
    PURGE=true
fi

echo "==> Uninstalling ${PROJECT}"

########################################
# Stop & disable systemd service
########################################

if command -v systemctl >/dev/null 2>&1; then
    systemctl --user stop "${PROJECT}.service" 2>/dev/null || true
    systemctl --user disable "${PROJECT}.service" 2>/dev/null || true
    systemctl --user daemon-reload
fi

########################################
# Remove service
########################################

if [[ -f "${SERVICE_FILE}" ]]; then
    rm -f "${SERVICE_FILE}"
    echo "Removed systemd service."
fi

########################################
# Remove binary
########################################

if [[ -f "${BIN_FILE}" ]]; then
    rm -f "${BIN_FILE}"
    echo "Removed binary."
fi

########################################
# Remove config (optional)
########################################

if [[ "${PURGE}" == true ]]; then
    rm -rf "${CONFIG_DIR}"
    echo "Removed configuration."
else
    echo "Keeping configuration:"
    echo "  ${CONFIG_DIR}"
    echo
    echo "Run with '--purge' to remove it."
fi

########################################
# Done
########################################

echo
echo "=========================================="
echo "Uninstallation completed."
echo "=========================================="