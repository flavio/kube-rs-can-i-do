Simple POC showing how to use the Kubernetes' [authorization API](https://kubernetes.io/docs/reference/access-authn-authz/authorization/#checking-api-access)
to check if a user/SA is allowed to perform a certain operation.

The code is using Rust, with the `kube` and the `k8s-opanapi` crates.

## Demo

Create a Kubernetes cluster, unless you already have one:

```console
kind create cluster
```

Create a ServiceAccount named `demo1` inside of the `default` namespace. This SA is allowed to list Secrets:

```console
kubectl apply -f create-sa-that-can-list-secrets.yaml
```

Run the code:

```console
cargo run
```
