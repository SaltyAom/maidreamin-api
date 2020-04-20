## Dreamin
Maidreamin API written in Rust and up-running with basic Docker and Kubernetes.

## Settings up
Creating a Rust `vendor` target:
```
cargo vendor .
```

Build Docker image:
```
docker build -t dreamin-api .
```

Run Kubernetes built pre-written:
```
kubectl create -f manifest.json
```

Expose an app:
```
kubectl expose deployments dreamin-api --type=LoadBalancer --name=dreamin
```

Check out a deployment:
```
curl localhost
```