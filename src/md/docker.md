## Docker

Why use Docker?
- Compatibility issues with different servers
- Libraries and dependencies among applications
- Long setup time for local environments

Containers are isolated environments with their own processes, network, and mounts. They share the same OS kernel. 

Dockers uses LXC containers; providing a high level tool for end-users to use them.

Containers vs. Virtual Machines
- Virtual machines contain:
	- Application
	- Library / Dependencies
	- OS
	- Managed by a Hypervisor and Hardware Infrastructure 
- Containers contain:
	- Applications 
	- Library / Dependencies
	- Managed by Docker, OS, Hardware Infrastructure

Containers consume less disk space. They can boot up faster. Docker has less isolation. VMs have complete isolation.

In actual environments, you have containers inside of virtual machines. 

Container vs. Image
- Containers run instances of images
- Developers develop applications and hand it to the operations team; this generate issues
- With Docker, we create a Docker file that creates an image and runs the same way anywhere
- Docker contributes to the DevOps culture

## Docker Commands
- `docker run nginx` - run container
	- -d run in the background
- `docker ps` - list containers
- `docker ps -a` - show all containers
- `docker stop image` - stop a given container
- `docker images` list all images
- `docker pull nginx` pull images
- `docker exec cat etc/hosts` show the list of hosts



