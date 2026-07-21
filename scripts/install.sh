#!/usr/bin/env bash

set -euo pipefail

PROJECT="hypr-download-sorter"

BIN_DIR="${HOME}/.local/bin"
CONFIG_DIR="${HOME}/.config/${PROJECT}"

SERVICE_DIR="${HOME}/.config/systemd/user"
SERVICE_FILE="${SERVICE_DIR}/${PROJECT}.service"

echo "==> Installing ${PROJECT}"

########################################
# Check cargo
########################################

if ! command -v cargo >/dev/null 2>&1; then
    echo
    echo "Error: Rust/Cargo is not installed."
    echo "Install Rust first:"
    echo "https://rustup.rs"
    exit 1
fi

########################################
# Build
########################################

echo "==> Building release binary..."

cargo build --release

########################################
# Install binary
########################################

mkdir -p "${BIN_DIR}"

install -Dm755 \
    "target/release/${PROJECT}" \
    "${BIN_DIR}/${PROJECT}"

echo "Installed binary."

########################################
# Install config
########################################

mkdir -p "${CONFIG_DIR}"

if [[ ! -f "${CONFIG_DIR}/default.toml" ]]; then
    cp config/default.toml \
        "${CONFIG_DIR}/default.toml"

    echo "Installed default configuration."
else
    echo "Keeping existing configuration."
fi

########################################
# Install systemd service
########################################

mkdir -p "${SERVICE_DIR}"

install -Dm644 \
    "systemd/${PROJECT}.service" \
    "${SERVICE_FILE}"

systemctl --user daemon-reload

systemctl --user enable --now "${PROJECT}.service"

echo "Installed systemd service."

########################################
# PATH check
########################################

case ":$PATH:" in
    *":${BIN_DIR}:"*) ;;
    *)
        echo
        echo "WARNING:"
        echo "${BIN_DIR} is not in your PATH."
        echo
        echo 'Add this line to your shell config:'
        echo
        echo 'export PATH="$HOME/.local/bin:$PATH"'
        ;;
esac

########################################
# Done
########################################

echo
echo "=========================================="
echo "Installation completed successfully."
echo
echo "Binary:"
echo "  ${BIN_DIR}/${PROJECT}"
echo
echo "Config:"
echo "  ${CONFIG_DIR}/default.toml"
echo
echo "Service:"
echo "  ${SERVICE_FILE}"
echo
echo "Status:"
echo "  systemctl --user status ${PROJECT}"
echo
echo "=========================================="