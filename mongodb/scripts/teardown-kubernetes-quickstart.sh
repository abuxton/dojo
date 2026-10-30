#!/usr/bin/env bash
set -euo pipefail

: "${KUBE_CONTEXT:=rancher-desktop}"
: "${KUBE_NAMESPACE:=mongodb}"
: "${HELM_RELEASE:=kubernetes-operator}"

require_command() {
  command -v "$1" >/dev/null 2>&1 ||
    { printf 'Required command not found: %s\n' "$1" >&2; exit 1; }
}

main() {
  require_command helm
  require_command kubectl
  kubectl config get-contexts "$KUBE_CONTEXT" >/dev/null ||
    { printf 'Kubernetes context not found: %s\n' "$KUBE_CONTEXT" >&2; exit 1; }

  if ! kubectl --context "$KUBE_CONTEXT" get namespace "$KUBE_NAMESPACE" >/dev/null 2>&1; then
    printf 'Namespace %s does not exist; nothing to tear down.\n' "$KUBE_NAMESPACE"
    return
  fi

  printf 'This deletes the %s namespace, including all MongoDB quickstart data and persistent-volume claims.\n' \
    "$KUBE_NAMESPACE"
  read -rp 'Type DELETE to continue: ' confirmation
  if [[ "$confirmation" != "DELETE" ]]; then
    printf 'Teardown cancelled.\n'
    return
  fi

  if helm --kube-context "$KUBE_CONTEXT" status "$HELM_RELEASE" \
    --namespace "$KUBE_NAMESPACE" >/dev/null 2>&1; then
    helm --kube-context "$KUBE_CONTEXT" uninstall "$HELM_RELEASE" \
      --namespace "$KUBE_NAMESPACE"
  fi

  kubectl --context "$KUBE_CONTEXT" delete namespace "$KUBE_NAMESPACE" \
    --wait=true \
    --timeout=300s
  printf 'MongoDB Kubernetes quickstart teardown completed.\n'
}

main "$@"
