//NoahMcGe_1/18/23
//https://github.com/NoahMcGe/node-manager
extern crate termion;
use std::process;
use termion::{color, style};
use std::io;
use std::io::{Write, Read, Seek, SeekFrom,};
use std::fs;
use std::fs::OpenOptions;
//use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;
use toml::{Value};
use std::error;
use std::fmt;
mod menus;

struct Commands {
    default: Vec<String>,
    custom: Vec<String>,
}

struct Connection {
    name: String,
    user: String,
    password: String,
    ip_address: String,
    port: i64,
}

#[derive(Debug)]
struct MyError {
    message: String,
}

impl MyError {
    fn new(message: &str) -> MyError {
        MyError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for MyError {}



fn main() {
    menus::clear_screen();
    menu1_options();
    
}

fn menu1_options(){
    check_and_create_files();
    let connections = read_connections_from_file("ssh_connections.toml");
    let commands = read_commands_from_file("ssh_commands.toml");
    //
    loop{
    menus::menu1();
    let opt_select = inputfunc();


    if opt_select == "Q" || opt_select == "q" {
        println!("\n{}    Quitting... {}", color::Fg(color::Red), color::Fg(color::White));
        process::exit(0x0100);
    }
//  - Help -
    else if opt_select == "H" || opt_select == "h" {
        menus::help();
    }
//  - About -
    else if opt_select == "A" || opt_select == "a" {
        menus::about();
    }
//  - Node Menu -
    else if opt_select == "N" || opt_select == "n" {
        menus::clear_screen();
        pre_config();
    }
    else if opt_select == "C" || opt_select == "c" {
        menus::clear_screen();
        loop{
            menus::connect_menu();
            for (i, connection) in connections.iter().enumerate() {
            print!("{}{}. {}\n    ",color::Fg(color::Cyan), i + 1, connection.name);
                }
            menus::connect_menu2();
            let opt_select = inputfunc();
            if opt_select == "Q" || opt_select == "q" || opt_select == "B" || opt_select == "b" {
                break;
            }
            else if opt_select == "C" || opt_select == "c"{
            // Command All Nodes
                loop{

                     println!("{}{}\n    - Commands -{}{}",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan));
                        print!("    C. Custom / Input\n    ");
                        for (i, command) in commands.default.iter().enumerate() {
                            print!("{}. {}\n    ",i+1, command);
                        }
                        menus::connect_menu2();
                        let opt_select = inputfunc();
                         //
                        menus::clear_screen();
                        if opt_select == "Q" || opt_select == "q" || opt_select == "B" || opt_select == "b"{
                            break;
                        }
                        else if opt_select == "C" || opt_select == "c"{
                            print!("\nType 'QUIT' to Exit");
                            loop{
                                print!("\nInput Command > ");
                                let opt_select = inputfunc();
                                if opt_select == "QUIT" || opt_select == "quit" {break;}
                                for (i, connection) in connections.iter().enumerate(){
                                    println!("{}. {}",i+1,connection.name);
                                    //connect_and_execute_no_input(connection, &opt_select);
                                    match connect_and_execute_no_input(connection, &opt_select) {
                                        Ok(()) => {},
                                        Err(e) => println!("Error: {}", e),
                                    }
                                }

                            }
                        }
                        else {
                            // Try to parse the user input as an integer.
                            let selection = match opt_select.parse::<usize>() {
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid selection");
                                break;
                                }
                            };
                            for (i, connection) in connections.iter().enumerate(){
                                println!("{}. {}",i+1,connection.name);
                                match connect_and_execute_no_input(connection, &commands.default[selection-1]) {
                                    Ok(()) => {},
                                    Err(e) => println!("Error: {}", e),
                                }
                                //connect_and_execute_no_input(connection, &commands.default[selection-1]);
                            }
                        }

                }
            }
            // Try to parse the user input as an integer.
            let selection = match opt_select.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection");
                continue;
                }
            };
            let connection = &connections[selection - 1];
            //
            loop{
                println!("{}{}\n    - Commands -{}{}",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan));
                print!("    C. Custom / Input\n    ");
                for (i, command) in commands.default.iter().enumerate() {
                    print!("{}. {}\n    ",i+1, command);
                }
                menus::connect_menu2();
                let opt_select = inputfunc();
                if opt_select == "Q" || opt_select == "q" || opt_select == "B" || opt_select == "b"{
                    break;
                }
                else if opt_select == "C" || opt_select == "c"{
                    print!("\nType 'QUIT' to Exit");
                    loop{
                        print!("\nInput Command > ");
                        let opt_select = inputfunc();
                        if opt_select == "QUIT" || opt_select == "quit" {break;}
                        //connect_and_execute_no_input(connection, &opt_select);
                        match connect_and_execute_no_input(connection, &opt_select) {
                            Ok(()) => {},
                            Err(e) => println!("Error: {}", e),
                        }

                    }
                }
                else {
                    // Try to parse the user input as an integer.
                    let selection = match opt_select.parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid selection");
                        break;
                        }
                    };
                    //
                    menus::clear_screen();
                    //connect_and_execute_no_input(connection, &commands.default[selection-1]);
                    match connect_and_execute_no_input(connection, &commands.default[selection-1]) {
                        Ok(()) => {},
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        //
        }
        }
    }
}

fn inputfunc()-> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input);
    input = input.trim().to_string();
    return input.to_string()
}

fn pre_config() {
    loop{
    menus::pre_config_men();
    let opt_select = inputfunc();
    //  - Back -
    if opt_select == "B" || opt_select == "b" {
        menus::clear_screen();
        break()
    }
    else if opt_select == "Q" || opt_select == "q" {
        menus::clear_screen();
        break()
    }
    else if opt_select == "P" || opt_select == "p" {
        menus::clear_screen();
        load_config();
        
    }
    else if opt_select == "A" || opt_select == "a" {
        menus::clear_screen();
        menus::add_node_1();
        let temp_node_name = inputfunc();
        menus::clear_screen();
        menus::add_node_2(&temp_node_name);
        let temp_node_user = inputfunc();
        menus::clear_screen();
        menus::add_node_3(&temp_node_name,&temp_node_user);
        let temp_node_pass = inputfunc();
        menus::clear_screen();
        menus::add_node_4(&temp_node_name,&temp_node_user,&temp_node_pass);
        let temp_node_ip = inputfunc();
        menus::clear_screen();
        menus::add_node_5(&temp_node_name,&temp_node_user,&temp_node_pass,&temp_node_ip);
        let temp_node_port = inputfunc();
        let temp_node_port = match temp_node_port.parse::<u16>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid selection, Port has to be a Integer");
                break;
                }
            };
        menus::clear_screen();
        menus::add_node_6(&temp_node_name,&temp_node_user,&temp_node_pass,&temp_node_ip,&temp_node_port);
        let opt_select = inputfunc();
        if opt_select == "Yes" || opt_select == "yes" || opt_select == "Y" || opt_select == "y" {
            add_connection("ssh_connections.toml", &temp_node_name, &temp_node_user, &temp_node_pass, &temp_node_ip, temp_node_port);
            main();
        }
        else {
            break;
        }

        
    }
    else if opt_select == "R" || opt_select == "r" {
        menus::clear_screen();
        println!("{:?}",get_connection_names("ssh_connections.toml"));
        menus::remove_node();
        let opt_select = inputfunc();
        if opt_select == "Q" || opt_select == "q" || opt_select == "B" || opt_select == "b"{break;}
        else {
            remove_connection(&opt_select);
        }
        
    }
    }

}

fn load_config() {
    // Read the contents of the toml file into a string.
    let contents = fs::read_to_string("ssh_connections.toml").unwrap();

    // Parse the contents of the file into a TOML value.
    let value = contents.parse::<Value>().unwrap();

    // Get the connections array from the TOML value.
    let connections = value["connections"].as_array().unwrap();

    // Iterate over the connections and print their details.
    for connection in connections {
        let name = connection["name"].as_str().unwrap();
        let user = connection["user"].as_str().unwrap();
        let password = connection["password"].as_str().unwrap();
        let ip_address = connection["ip_address"].as_str().unwrap();
        let port = connection["port"].as_integer().unwrap();

        println!("name: {}", name);
        println!("user: {}", user);
        println!("password: {}", password);
        println!("ip_address: {}", ip_address);
        println!("port: {}\n", port);
    }
}

fn read_connections_from_file(filename: &str) -> Vec<Connection> {
    // Read the contents of the toml file into a string.
    let contents = fs::read_to_string(filename).unwrap();

    // Parse the contents of the file into a TOML value.
    let value = contents.parse::<Value>().unwrap();

    // Get the connections array from the TOML value.
    let connections = value["connections"].as_array().unwrap();

    // Convert the TOML value into a vector of Connection objects.
    let mut result = Vec::new();
    for connection in connections {
        let name = connection["name"].as_str().unwrap().to_owned();
        let user = connection["user"].as_str().unwrap().to_owned();
        let password = connection["password"].as_str().unwrap().to_owned();
        let ip_address = connection["ip_address"].as_str().unwrap().to_owned();
        let port = connection["port"].as_integer().unwrap();

        result.push(Connection {
            name,
            user,
            password,
            ip_address,
            port,
        });
    }

    result
}

fn read_commands_from_file(filename: &str) -> Commands {
    // Read the contents of the toml file into a string.
    let contents = fs::read_to_string(filename).unwrap();

    // Parse the contents of the file into a TOML value.
    let value = contents.parse::<Value>().unwrap();

    // Extract the default and custom values from the TOML value.
    let default = value["commands"][0]["default"].as_array().unwrap().iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let custom = value["commands"][0]["custom"].as_array().unwrap().iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    // Create a Commands object with the extracted values.
    let commands = Commands { default, custom };
    commands
}

fn connect_and_execute_no_input(connection: &Connection, command: &str) -> Result<(), MyError> {
    // Convert the IP address and port to the correct types.
    let ip_address: String = connection.ip_address.parse().map_err(|_| MyError::new("Invalid IP address"))?;
    let port: u16 = connection.port.try_into().map_err(|_| MyError::new("Invalid port"))?;

    // Connect to the SSH server.
    let tcp = TcpStream::connect((ip_address, port)).map_err(|e| MyError::new(&e.to_string()))?;
    let mut sess = Session::new().map_err(|e| MyError::new(&e.to_string()))?;
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(|e| MyError::new(&e.to_string()))?;

    // Authenticate using the password.
    sess.userauth_password(&connection.user, &connection.password).map_err(|e| MyError::new(&e.to_string()))?;

    // Execute the command.
    let mut channel = sess.channel_session().map_err(|e| MyError::new(&e.to_string()))?;
    channel.exec(command).map_err(|e| MyError::new(&e.to_string()))?;

     // Read the output of the command into a string.
    let mut output = String::new();
    channel.read_to_string(&mut output).map_err(|e| MyError::new(&e.to_string()))?;

    // Print the output to the terminal.
    println!("{}", output);
    Ok(())
}


fn add_connection(file_path: &str, name: &str, user: &str, password: &str, ip_address: &str, port: u16) {
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut value: toml::Value = toml::from_str(&contents).unwrap();
    let connections = value.get_mut("connections").unwrap();
    let mut new_connection = toml::map::Map::new();
    new_connection.insert("name".to_string(), toml::Value::String(name.to_string()));
    new_connection.insert("user".to_string(), toml::Value::String(user.to_string()));
    new_connection.insert("password".to_string(), toml::Value::String(password.to_string()));
    new_connection.insert("ip_address".to_string(), toml::Value::String(ip_address.to_string()));
    new_connection.insert("port".to_string(), toml::Value::Integer(port as i64));
    let mut connections_vec = connections.as_array_mut().unwrap();
    connections_vec.push(toml::Value::Table(new_connection));
    let new_contents = toml::to_string(&value).unwrap();
    let mut file = OpenOptions::new().write(true).open(file_path).unwrap();
    file.write_all(new_contents.as_bytes()).unwrap();
}


fn remove_connection(name: &str) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("ssh_connections.toml")
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut toml: Value = toml::from_str(&contents).unwrap();

    let connections = toml["connections"].as_array_mut().unwrap();
    connections.retain(|connection| {
        connection["name"].as_str().unwrap() != name
    });

    let updated_contents = toml::to_string_pretty(&toml).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    file.write_all(updated_contents.as_bytes()).unwrap();
    file.set_len(updated_contents.len() as u64).unwrap();
}


fn get_connection_names(file_path: &str) -> Vec<String> {
    let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let value: toml::Value = toml::from_str(&contents).unwrap();
    let connections = value.get("connections").unwrap();
    let connections_vec = connections.as_array().unwrap();
    let mut names: Vec<String> = vec![];
    for connection in connections_vec {
        let name = connection.get("name").unwrap().as_str().unwrap();
        names.push(name.to_string());
    }
    return names;
}

fn check_and_create_files() {
    let ssh_connections_path = "ssh_connections.toml";
    let ssh_commands_path = "ssh_commands.toml";
    let default_connections_text = "[[connections]]\nname = \"Miner1\"\nuser = \"login\"\npassword = \"secret123\"\nip_address = \"192.168.1.12\"\nport = 22\n";
    let default_commands_text = "[[commands]]\ndefault = [\"uptime\",\"neofetch\"]\ncustom = [\"\"]\n";

    if !fs::metadata(ssh_connections_path).is_ok() {
        match fs::File::create(ssh_connections_path) {
            Ok(mut file) => {
                file.write_all(default_connections_text.as_bytes())
                    .expect("Failed to write default connections to file.");
                println!("File {} created with default connections.", ssh_connections_path);
            }
            Err(e) => eprintln!("Error creating file {}: {}", ssh_connections_path, e),
        }
    }

    if !fs::metadata(ssh_commands_path).is_ok() {
        match fs::File::create(ssh_commands_path) {
            Ok(mut file) => {
                file.write_all(default_commands_text.as_bytes())
                    .expect("Failed to write default commands to file.");
                println!("File {} created with default commands.", ssh_commands_path);
            }
            Err(e) => eprintln!("Error creating file {}: {}", ssh_commands_path, e),
        }
    }
}