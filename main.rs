use std::io;

fn main() {
    println!("");
    println!("_______________________________________________________________________________________________");
    println!("");
    println!("This is the Terminal Website of: ");
    println!("     ██╗ ██████╗ ███████╗███████╗██╗  ██╗███╗   ███╗ ██████╗ ██╗  ██╗███████╗ ██████╗ 
     ██║██╔═══██╗██╔════╝██╔════╝██║  ██║████╗ ████║██╔═══██╗██║  ██║██╔════╝██╔════╝ 
     ██║██║   ██║█████╗  ███████╗███████║██╔████╔██║██║   ██║███████║███████╗███████╗ 
██   ██║██║   ██║██╔══╝  ╚════██║██╔══██║██║╚██╔╝██║██║   ██║╚════██║╚════██║██╔═══██╗
╚█████╔╝╚██████╔╝███████╗███████║██║  ██║██║ ╚═╝ ██║╚██████╔╝     ██║███████║╚██████╔╝
╚════╝  ╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝      ╚═╝╚══════╝ ╚═════╝ 
    ");
    println!("Please type 'help' for a list of commands");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let com :&str = input.trim();

    let mut helpexit: bool = false;
    
    while helpexit == false{
        match com {
            "help" => {help(); helpexit = true},
            "projects" => {projects(); helpexit = true},
            "whosisthis" => {whosisthis(); helpexit = true},
            "links" => {links(); helpexit = true},
            "resources" => {resources(); helpexit = true},
            "whatisthis" => {whatisthis(); helpexit = true},
            _ => println!("That wasn't one of the options!")
     }
    }
}
fn help(){
    println!("
    1- projects : gives a list of my projects
    2- whosisthis : gives a description of the creator of this project
    3- links : gives links to relavent the creator of this project
    4- resources : links to sources used to help make this project
    5- whatisthis : gives a description of this project");
    
}
fn projects(){
    println!("You typed projects!");
}
fn whosisthis(){
    println!("You typed whosisthis!");
}
fn links(){
    println!("You typed links!");
}
fn resources(){
    println!("You typed resources!");
}
fn whatisthis(){
    println!("You typed whatisthis!");
}
