# IaaC demo

Deployment of a Rust + HTMX application that uses a MySQL database to store data. The repository contains Terraform scripts that create the required resources in a Minikube cluster and deployment files that deploy the application and database into the cluster. There is also a second deployment of a simple NGINX server that only serves static data.

## Requirements

An up-and-running [minikube](https://minikube.sigs.k8s.io/docs/) cluster with context named miniube in the ~/.kube/config and ingress addon enabled.

```bash
minikube addons enable ingress
```

A generated SSL certificate and key stored as tls.crt and tls.key in the root of the repository.

```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout tls.key -out tls.crt
```

## Usage

```bash
# Create Kubernetes resources
terraform -chdir=terraform apply
```

```bash
# Deploy the app and create the database
kubectl apply -f manifests/deployment.yaml -n app
```

```bash
# To access Minikube ingress
minikube tunnel
```

To access the app on https://web-app.local/ you need to edit the /etc/hosts file as follows.

```bash
# To access web app
127.0.0.1 web-app.local

# To access static web page
127.0.0.1 static-web.local
```

## Future work

Use PaaS database instead of an instance running in a container.

Use Vault to store required secrets.

Create CI/CD pipeline.

Handle transient errors in the application.
