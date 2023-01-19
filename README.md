# Node Manager,
<h2>Designed for Debian and forks.</h2>
I made this to help my brothers (who don't know how to terminal) manage their miners, but it will work for any node with ssh.
    You must have both config files (ssh_commands.toml,ssh_connections.toml)
    *The program should automagically create the toml files with default examples if files dont exist.*
    in the same localtion as the executable to run.
    
    The Default commands can be changed through the "ssh_commands.toml" file.
    Example:
    default = ["uptime","neofetch"]
    changed to:
    default = ["ls","neofetch","pwd","ip a","ping 8.8.8.8"]

<h1>This stores a password in <b>PLAIN TEXT</b></h1>
    <h2>ssh_connections.toml example:</h2>
    
    [[connections]]
    name = "Miner1"
    user = "login"
    password = "secret123"
    ip_address = "192.168.1.12"
    port = 22

![alt text](https://github.com/NoahMcGe/node-manager/blob/main/img/start.png)
![alt text](https://github.com/NoahMcGe/node-manager/blob/main/img/connect.png)
![alt text](https://github.com/NoahMcGe/node-manager/blob/main/img/add_node.png)

<h3>To Compile:</h3>
    
    git clone https://github.com/NoahMcGe/node-manager
    cd node-manager
    cargo run
