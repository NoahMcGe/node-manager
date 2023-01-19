//NoahMcGe_1/18/23
//https://github.com/NoahMcGe/node-manager
extern crate termion;
use termion::{color, style};

pub fn menu1() {
	print!("\n{}{}    - Menu -{}{}
    C. Connect
    N. Nodes
   --
    H. Help
    A. About
    Q. Quit
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::White));
}

pub fn help() {
    clear_screen();
    println!("{}{}    - Help -{}{}\n
    Coming Soon!{}",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::White));
}

pub fn about() {
    clear_screen();
    println!("{}{}    - About -{}{}\n
    Coming Soon!{}",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::White));
}

pub fn pre_config_men() {
    print!("\n{}{}    - Nodes -{}{}
    P. Print Config
    A. Add Node
    R. Remove Node
   --
    Q. Back
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::White));

}

pub fn connect_menu() {
    print!("\n{}{}    - Nodes -{}{}
    C. Command All Nodes\n    ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan));
    // Num. Name of node
}
pub fn connect_menu2() {
    print!("{}--
    Q. Back
    {}
    >  ",color::Fg(color::Cyan),color::Fg(color::White));
}

pub fn add_node_1() {
    print!("\n{}{}    - Add Node -{}{}
    Node Name: 
    {}Username:
    Password: 
    IP Address: 
    Port(Default 22): 
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Green),color::Fg(color::Cyan),color::Fg(color::White));
}
pub fn add_node_2(node_name: &str) {
    print!("\n{}{}    - Add Node -{}{}
    Node Name: {node_name}
    {}Username: 
    {}Password: 
    IP Address: 
    Port(Default 22): 
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::Green),color::Fg(color::Cyan),color::Fg(color::White));
}
pub fn add_node_3(node_name: &str, node_user: &str) {
    print!("\n{}{}    - Add Node -{}{}
    Node Name: {node_name}
    Username: {node_user} 
    {}Password: 
    {}IP Address: 
    Port(Default 22): 
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::Green),color::Fg(color::Cyan),color::Fg(color::White));
}
pub fn add_node_4(node_name: &str, node_user: &str, node_pass: &str) {
    print!("\n{}{}    - Add Node -{}{}
    Node Name: {node_name}
    Username: {node_user} 
    Password: {node_pass}
    {}IP Address: 
    {}Port(Default 22): 
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::Green),color::Fg(color::Cyan),color::Fg(color::White));
}
pub fn add_node_5(node_name: &str, node_user: &str, node_pass: &str, node_ip: &str ) {
    print!("\n{}{}    - Add Node -{}{}
    Node Name: {node_name}
    Username: {node_user} 
    Password: {node_pass}
    IP Address: {node_ip}
    {}Port(Default 22): 
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::Green),color::Fg(color::White));
}
pub fn add_node_6(node_name: &str, node_user: &str, node_pass: &str, node_ip: &str, node_port: &u16 ) {
    print!("\n{}{}    - Add Node -{}{}
    Node Name: {node_name}
    Username: {node_user} 
    Password: {node_pass}
    IP Address: {node_ip}
    Port(Default 22): {node_port}
    {}  Is This Correct? (Y/N):
    {}
    >  ",style::Bold,color::Fg(color::Blue),style::NoBold,color::Fg(color::Cyan),color::Fg(color::Green),color::Fg(color::White));
}
pub fn remove_node() {
    print!("{}--
    Q. Back
    {}  Type the name of the Node to be Removed.
    {}
    >  ",color::Fg(color::Cyan),color::Fg(color::Green),color::Fg(color::White));
}


pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}