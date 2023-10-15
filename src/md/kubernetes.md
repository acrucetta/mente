## Kubernetes

Came with the rise of microservices; we now use dozens of containers across different environments 

Kubernetes guarantees:
- high availability or no downtime
- scalability or high performance
- disaster recovery - backup and restore

Basic architecture:
- Linux Master Node (lightweight)
	- Worker node (heavy weight) with "kubelet" processes inside each of them
	- Each worker node has containers with applications running inside of them
- The master node runs Kubernetes processes which include: 
	- API server (entry point to K8S cluster). 
	- Controller manager - keeps track of what's happening in the cluster
	- Scheduler - ensures pod placement 
	- etcd - key value storage; current state 
- Virtual network unified by a powerful machine connected to all of the nodes

 