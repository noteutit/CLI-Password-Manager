use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt (name='Password_manager')]

enum Commands{
    Add{
        service:String,
        user_name:String,
        password: String,
    },
    Retrieve{
        service:String,
    },
    Delete{
        service:String,
    },
    Generate{
        password_length=usize,
    } 


    }

fn main() {
    let opt = Commands::new();
    match opt {
        Commands::Add{service,user_name,password} =>{
            println!("This is to add password for service,{}",service);
        }
        Commands::Retrieve{service}=> {
        println!("Retrieving password for :{}",service);
        }
        Commands::Delete{service} => {
            println!("Deleting password for : {}",service);            
        }
        Commands::Generate{password_length} =>{
            println!("Generating password for: {}",service);
        }
    }
    mod password_generation;
} 

