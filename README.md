# What the header!?!

Super simple docker image for checking headers.

Available to run with `steveyackey/whattheheader:latest`

Run it in kubernetes:
```bash
kubectl create deploy whattheheader --image=steveyackey/whattheheader  

# Expose through a ClusterIP
kubectl expose deploy whattheheader --port=80 --target-port=8080 --type=ClusterIP

# Expose through a load balancer
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
