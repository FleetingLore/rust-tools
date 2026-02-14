use std::io;

fn input() -> String {
    println!("select an option:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn menu() -> String {
    println!("
#===================<Pytools>===================#
#                                               #
# C.checkra1n                                   #
# P.palera1n                                    #
# U.UsbToolBox                                  #
# G.GenSMBIOS                                   #
# T.ProperTree                                  #
# Q.Quit                                        #
#===================<Made By>===================#
#                                               #
# bilibili: SingGin                             #
#                                               #
#===============================================#
");
    input()
}

fn checkra1n() -> String {
    println!("
#==================<checkra1n>==================#
#                                               #
# C.checkra1n                                   #
# P.palera1n                                    #
# U.UsbToolBox                                  #
#===============================================#
");
    input()
}

fn welcome() {
    println!("Welcome to Pytools! You can use it as a utilities.")
}

fn main() {
    let mut pq = String::new();
    println!("Hello, world!");

}