# MongoDB Kubernetes quickstart

These manifests and scripts make the [MongoDB Controllers for Kubernetes Kind
quickstart](https://www.mongodb.com/docs/kubernetes/current/kind-quick-start/)
repeatable on Rancher Desktop. They are evaluation-only: the quickstart deploys
Ops Manager and a three-member application database on one local Kubernetes
node, so it is not production-ready.

Before deploying, allocate at least 8 GB of memory and 4 CPUs to Rancher
Desktop Kubernetes. The Ops Manager pod alone requests 5 GiB. If it remains
`Pending`, inspect its scheduling event:

```sh
kubectl --context rancher-desktop --namespace mongodb describe pod ops-manager-0
```

The current local deployment reports `Insufficient memory`; increasing Rancher
Desktop's Kubernetes memory allocation is required before it can start.

## Guided manual execution

Run the do-nothing guide for the complete manual procedure:

```sh
./scripts/deploy-kubernetes-quickstart-guide.sh
```

It only prints each command and waits for confirmation. It does not change the
cluster.

## Repeatable deployment

The deployment script uses `helm upgrade --install` and declarative
`kubectl apply`, so it can be rerun safely with the same inputs. It never
prints credentials.

```sh
export OPS_MANAGER_ADMIN_USERNAME="admin@example.test"
read -rsp "Ops Manager password: " OPS_MANAGER_ADMIN_PASSWORD; export OPS_MANAGER_ADMIN_PASSWORD; echo
export OPS_MANAGER_ADMIN_FIRST_NAME="Admin"
export OPS_MANAGER_ADMIN_LAST_NAME="User"

KUBE_CONTEXT=rancher-desktop ./scripts/deploy-kubernetes-quickstart.sh
```

After Ops Manager is running, create an organization API key in its UI. Export
the organization ID, API keys, and a password for the database user, then
rerun the script to apply the ConfigMap, credential Secrets, replica set, and
database user:

```sh
export OPS_MANAGER_ORG_ID="..."
export OPS_MANAGER_PUBLIC_KEY="..."
read -rsp "Ops Manager private key: " OPS_MANAGER_PRIVATE_KEY; export OPS_MANAGER_PRIVATE_KEY; echo
read -rsp "Database user password: " MONGODB_USER_PASSWORD; export MONGODB_USER_PASSWORD; echo

KUBE_CONTEXT=rancher-desktop ./scripts/deploy-kubernetes-quickstart.sh
```

The script accepts `KUBE_NAMESPACE`, `HELM_RELEASE`, and
`OPS_MANAGER_TIMEOUT_SECONDS` when a non-default namespace, Helm release, or
readiness timeout is needed.
