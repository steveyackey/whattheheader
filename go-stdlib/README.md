# What the header!?!

Super simple docker image for checking headers.

Available to run with `steveyackey/whattheheader:latest`

Run it locally:
```bash
go run main.go
```

Run it in Docker:
```bash
# Run it and listen on port 8080
docker run --rm -p 8080:8080 steveyackey/whattheheader:latest
```

Run it in kubernetes:
```bash
kubectl create deploy whattheheader --image=steveyackey/whattheheader  

# Expose through a ClusterIP, with service listening on port 80
kubectl expose deploy whattheheader --port=80 --target-port=8080 --type=ClusterIP

# Expose through a load balancer, lb listens on port 80
kubectl expose deploy whattheheader --port=80 --target-port=8080 --type=LoadBalancer
```

Example ingress:
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: whattheheader
  labels:
    name: whattheheader
spec:
  rules:
  - host: whattheheader.example.com
    http:
      paths:
      - pathType: Prefix
        path: "/"
        backend:
          service:
            name: whattheheader
            port: 
              number: 80
```
