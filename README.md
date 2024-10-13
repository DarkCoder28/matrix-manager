# Development Notes
## Port Forwarding to VM
- To forward a port to a vm from your machine, adapt the following command:
    - `ssh -L HOST_PORT:DEST_IP:DEST_PORT DEST_IP`
    - Ex: `ssh -L 4000:192.168.122.30:4000 virtdev`