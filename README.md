# IaaC demo

Deployment of a rust + htmx application that is using with mysql database to store data. Repository contains terraform scripts that creates required resources in a minikube cluster, deployment files that will deploy the application and databse into the cluster.

## Requirements

Up and running [minikube](https://minikube.sigs.k8s.io/docs/) cluster with context named miniube in the ~/.kube/config and ingress addon enabled.

```bash
minikube addons enable ingress
```

Generated ssl certificate and a key stored in a tls.crt and tls.key in a root of the repository.

```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout tls.key -out tls.crt
```

## Usage

```bash
# Create kubernetes resources
terraform -chdir=terraform apply
```

```bash
# Deploy app and create database
kubectl apply -f manifests/deployment.yaml -n app
```

```bash
# To access minikube ingress
minikube tunnel
```

To access the app on https://web-app.local/ we need to edit the /etc/hosts as well.

```bash
127.0.0.1 web-app.local
```

## Feature work

TODO: terraform create db in a cloud (PaaS)

TODO: move secrets to vault
