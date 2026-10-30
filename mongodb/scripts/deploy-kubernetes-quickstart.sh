#!/usr/bin/env bash
set -euo pipefail

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repository_dir=$(CDPATH= cd -- "$script_dir/.." && pwd)

: "${KUBE_CONTEXT:=rancher-desktop}"
: "${KUBE_NAMESPACE:=mongodb}"
: "${HELM_RELEASE:=kubernetes-operator}"
: "${OPS_MANAGER_TIMEOUT_SECONDS:=1800}"

require_command() {
  command -v "$1" >/dev/null 2>&1 ||
    { printf 'Required command not found: %s\n' "$1" >&2; exit 1; }
}

require_variable() {
  if [[ -z "${!1:-}" ]]; then
    printf 'Required environment variable is not set: %s\n' "$1" >&2
    exit 1
  fi
}

kubectl_in_namespace() {
  kubectl --context "$KUBE_CONTEXT" --namespace "$KUBE_NAMESPACE" "$@"
}

apply_secret() {
  local secret_name=$1
  shift
  kubectl_in_namespace create secret generic "$secret_name" "$@" \
    --dry-run=client --output=yaml | kubectl_in_namespace apply --filename=-
}

wait_for_ops_manager() {
  local deadline application_database_phase ops_manager_phase
  deadline=$((SECONDS + OPS_MANAGER_TIMEOUT_SECONDS))

  while (( SECONDS < deadline )); do
    application_database_phase=$(
      kubectl_in_namespace get opsmanager ops-manager \
        --output=jsonpath='{.status.applicationDatabase.phase}'
    )
    ops_manager_phase=$(
      kubectl_in_namespace get opsmanager ops-manager \
        --output=jsonpath='{.status.opsManager.phase}'
    )

    if [[ "$application_database_phase" == "Running" &&
      "$ops_manager_phase" == "Running" ]]; then
      return
    fi

    if [[ "$application_database_phase" == "Failed" ||
      "$ops_manager_phase" == "Failed" ]]; then
      printf 'Ops Manager failed to become ready (application database: %s, Ops Manager: %s).\n' \
        "${application_database_phase:-Pending}" \
        "${ops_manager_phase:-Pending}" >&2
      kubectl_in_namespace describe opsmanager ops-manager >&2
      exit 1
    fi

    printf 'Waiting for Ops Manager (application database: %s, Ops Manager: %s)...\n' \
      "${application_database_phase:-Pending}" \
      "${ops_manager_phase:-Pending}"
    sleep 10
  done

  printf 'Ops Manager did not become ready within %s seconds.\n' \
    "$OPS_MANAGER_TIMEOUT_SECONDS" >&2
  kubectl_in_namespace describe opsmanager ops-manager >&2
  exit 1
}

deploy_ops_manager() {
  require_variable OPS_MANAGER_ADMIN_USERNAME
  require_variable OPS_MANAGER_ADMIN_PASSWORD
  require_variable OPS_MANAGER_ADMIN_FIRST_NAME
  require_variable OPS_MANAGER_ADMIN_LAST_NAME

  helm repo add mongodb https://mongodb.github.io/helm-charts
  helm repo update mongodb
  helm upgrade --install "$HELM_RELEASE" mongodb/mongodb-kubernetes \
    --kube-context "$KUBE_CONTEXT" \
    --namespace "$KUBE_NAMESPACE" \
    --create-namespace \
    --wait \
    --timeout 10m

  kubectl --context "$KUBE_CONTEXT" create namespace "$KUBE_NAMESPACE" \
    --dry-run=client --output=yaml |
    kubectl --context "$KUBE_CONTEXT" apply --filename=-
  apply_secret ops-manager-admin-secret \
    --from-literal="Username=$OPS_MANAGER_ADMIN_USERNAME" \
    --from-literal="Password=$OPS_MANAGER_ADMIN_PASSWORD" \
    --from-literal="FirstName=$OPS_MANAGER_ADMIN_FIRST_NAME" \
    --from-literal="LastName=$OPS_MANAGER_ADMIN_LAST_NAME"
  kubectl_in_namespace apply --filename="$repository_dir/kubernetes/ops-manager.yaml"

  wait_for_ops_manager
  printf 'Ops Manager URL: '
  kubectl_in_namespace get opsmanager ops-manager \
    --output=jsonpath='{.status.opsManager.url}'
  printf '\n'
}

deploy_replica_set_when_configured() {
  local required_variables=(
    OPS_MANAGER_ORG_ID
    OPS_MANAGER_PUBLIC_KEY
    OPS_MANAGER_PRIVATE_KEY
    MONGODB_USER_PASSWORD
  )
  local variable

  for variable in "${required_variables[@]}"; do
    if [[ -z "${!variable:-}" ]]; then
      printf '\nOps Manager is ready. To deploy the replica set, set %s and rerun this script.\n' \
        "$variable"
      return
    fi
  done

  kubectl_in_namespace create configmap my-project \
    --from-literal="baseUrl=http://ops-manager-svc.${KUBE_NAMESPACE}.svc.cluster.local:8080" \
    --from-literal="projectName=my-project" \
    --from-literal="orgId=$OPS_MANAGER_ORG_ID" \
    --dry-run=client --output=yaml | kubectl_in_namespace apply --filename=-
  apply_secret organization-secret \
    --from-literal="publicKey=$OPS_MANAGER_PUBLIC_KEY" \
    --from-literal="privateKey=$OPS_MANAGER_PRIVATE_KEY"
  apply_secret mms-user-1-password \
    --from-literal="password=$MONGODB_USER_PASSWORD"
  kubectl_in_namespace apply \
    --filename="$repository_dir/kubernetes/replica-set.yaml" \
    --filename="$repository_dir/kubernetes/database-user.yaml"
}

main() {
  require_command helm
  require_command kubectl
  kubectl config get-contexts "$KUBE_CONTEXT" >/dev/null ||
    { printf 'Kubernetes context not found: %s\n' "$KUBE_CONTEXT" >&2; exit 1; }

  deploy_ops_manager
  deploy_replica_set_when_configured
}

main "$@"
