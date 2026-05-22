#!/bin/bash

set -euo pipefail

export PATH="$HOME/.local/bin:$PATH"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ANSIBLE_DIR="${SCRIPT_DIR}/ansible"
PLAYBOOK="${ANSIBLE_DIR}/playbooks/deploy.yml"
ANSIBLE_PLAYBOOK_BIN="$(command -v ansible-playbook || true)"

if [[ -z "${ANSIBLE_PLAYBOOK_BIN}" ]]; then
  echo "ansible-playbook is required. Please install Ansible first."
  exit 1
fi

ANSIBLE_PYTHON="$(dirname "$(readlink -f "${ANSIBLE_PLAYBOOK_BIN}")")/python"

cd "${ANSIBLE_DIR}"
"${ANSIBLE_PLAYBOOK_BIN}" "${PLAYBOOK}" \
  --tags "prereqs,keycloak_bootstrap" \
  -e "ansible_python_interpreter=${ANSIBLE_PYTHON}" \
  "$@"
