#!/usr/bin/env bash
set -euo pipefail

KUBE_CONTEXT=""

wait_for_enter() {
  read -rp "    Press Enter when done... " _
  printf '\n'
}

collect_context() {
  printf '==> Collecting context\n'
  read -rp "    Kubernetes context [rancher-desktop]: " KUBE_CONTEXT
  KUBE_CONTEXT=${KUBE_CONTEXT:-rancher-desktop}
  printf '\n'
}

step_1_allocate_rancher_desktop_resources() {
  printf '==> Step 1: Allocate Rancher Desktop resources\n'
  printf '%s\n' '    In Rancher Desktop, allocate at least 8 GB memory and 4 CPUs to Kubernetes.'
  printf '%s\n' '    Restart Kubernetes if Rancher Desktop requests it.'
  wait_for_enter
}

step_2_prepare_deployment_credentials() {
  printf '==> Step 2: Prepare Ops Manager credentials\n'
  printf '%s\n' '    Choose values for the initial Ops Manager administrator. Do not save them in the repository.'
  printf '%s\n' '    Export them in this shell:'
  printf '%s\n' '        export OPS_MANAGER_ADMIN_USERNAME="admin@example.test"'
  printf '%s\n' '        read -rsp "Ops Manager password: " OPS_MANAGER_ADMIN_PASSWORD; export OPS_MANAGER_ADMIN_PASSWORD; echo'
  printf '%s\n' '        export OPS_MANAGER_ADMIN_FIRST_NAME="Admin"'
  printf '%s\n' '        export OPS_MANAGER_ADMIN_LAST_NAME="User"'
  wait_for_enter
}

step_3_deploy_ops_manager() {  # TODO: automate
  printf '==> Step 3: Deploy the operator and Ops Manager\n'
  printf '%s\n' '    Run:'
  printf '        KUBE_CONTEXT=%q ./scripts/deploy-kubernetes-quickstart.sh\n' "$KUBE_CONTEXT"
  printf '%s\n' '    The command is idempotent. It installs or upgrades the Helm release, applies the secret,'
  printf '%s\n' '    and waits until the Ops Manager application database and application are Running.'
  wait_for_enter
}

step_4_create_ops_manager_api_credentials() {
  printf '==> Step 4: Create Ops Manager API credentials\n'
  printf '%s\n' '    If the deployment completed, port-forward the service in a separate terminal:'
  printf '        kubectl --context %q --namespace mongodb port-forward svc/ops-manager-svc-ext 8080:8080\n' \
    "$KUBE_CONTEXT"
  printf '%s\n' '    Open http://localhost:8080, sign in, and create an organization API key.'
  printf '%s\n' '    Record the organization ID, public key, and private key securely.'
  wait_for_enter
}

step_5_deploy_replica_set() {  # TODO: automate
  printf '==> Step 5: Deploy the replica set and database user\n'
  printf '%s\n' '    Export the values from Ops Manager and a new database-user password:'
  printf '%s\n' '        export OPS_MANAGER_ORG_ID="..."'
  printf '%s\n' '        export OPS_MANAGER_PUBLIC_KEY="..."'
  printf '%s\n' '        read -rsp "Ops Manager private key: " OPS_MANAGER_PRIVATE_KEY; export OPS_MANAGER_PRIVATE_KEY; echo'
  printf '%s\n' '        read -rsp "Database user password: " MONGODB_USER_PASSWORD; export MONGODB_USER_PASSWORD; echo'
  printf '%s\n' '    Rerun:'
  printf '        KUBE_CONTEXT=%q ./scripts/deploy-kubernetes-quickstart.sh\n' "$KUBE_CONTEXT"
  wait_for_enter
}

main() {
  printf '\n'
  printf '%s\n' 'Do-Nothing Procedure: MongoDB Kubernetes quickstart'
  printf '%s\n\n' 'This guide only prints instructions; it never deploys resources itself.'

  collect_context
  step_1_allocate_rancher_desktop_resources
  step_2_prepare_deployment_credentials
  step_3_deploy_ops_manager
  step_4_create_ops_manager_api_credentials
  step_5_deploy_replica_set

  printf 'Done.\n'
}

main "$@"
