use std::fs;
use std::path::Path;
use std::os::unix::fs as unix_fs;
use clap::{Parser, Subcommand};

#[derive(Parser)] //automatically generate code to parse command line arguments, Rust writes boilerplate for me :p
#[command(name = "crow")]
#[command(about = "Scarecrow is a CLI tool for managing development environments 🐦‍⬛", long_about = None)]
struct Cli { //defines my command line interface, the struct will hold the parsed arguments
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)] //automatically generate code to parse subcommands
enum Commands {
    Guard { //install and switch to env
        tool_ver: String, //ver of tool to use e.g. node 18.16.0
    },
    Pick { //change env
        tool_ver: String, //ver of tool to use e.g. node 18.16.0
    },
    Scare { //remove env
        tool_ver: String, //ver of tool to remove e.g. node 18.16.0
    },
    All, //list all envs
}

//helpers
fn symlink(tool : &str, version : &str) -> Result<(), Box<dyn std::error::Error>> {
    //creating symlink directory, for the tool the user is using scarecrow to install
    let symlink_dir = format!("{}/.scarecrow/bin", std::env::var("HOME")?);
    fs::create_dir_all(&symlink_dir)?; //only created if path does not exist

    let symlink_path = Path::new(&symlink_dir).join(tool); //where the symlink file will be
    let target = format!("../versions/{}/{}/bin/{}", tool, version, tool); //what the symlink points to
    
    if symlink_path.exists() { //remove old symlinks
        fs::remove_file(&symlink_path)?;
    }

    unix_fs::symlink(&target, &symlink_path)?; //create the symlink
    Ok(())
}

fn tar_install(tar_url : &str, path : &str) -> Result<(), Box<dyn std::error::Error>> {

}

fn guard(input : String) -> Result<(), Box<dyn std::error::Error>> {
    //download link example: https://nodejs.org/dist/v24.17.0/node-v24.17.0-darwin-arm64.tar.gz
    //download link structure: https://nodejs.org/dist/v{version}}/node-v{version}-{os}-{arch}.tar.gz  

    let (tool, ver) = input.split_once("@")
        .ok_or("Invalid format: use 'tool@version'")?;
    
    let base_path = format!("{}/.scarecrow/versions", std::env::var("HOME")?);
    let path = Path::new(&base_path); //convert to Path type

    println!("Base path: {}", base_path); 

    let tool_dir = path.join(tool);
    let ver_dir = tool_dir.join(ver); 

    fs::create_dir_all(&ver_dir)?; //creates the tool directory by joining path, tool name and version

    let bin_dir = ver_dir.join("bin"); //for bin
    fs::create_dir_all(&bin_dir)?;

    let bin_path = bin_dir.join(tool);
    fs::write(&bin_path, format!("fake binary for {} {}", tool, ver))?;

    println!("{}", std::env::consts::ARCH);
    println!("{}", std::env::consts::OS);

    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;

    let os_tarball = match os {
        "macos" => "darwin",
        "windows" => "win",
        "linux" => "linux",
        _ => "unknown",
    };

    let arch_tarball = match arch {
        "aarch64" => "arm64",
        "x86_64" => "x64",
        _ => "unknown",
    };

    let tarball = format!("https://nodejs.org/dist/v{}/node-v{}-{}-{}.tar.gz", ver, ver, os_tarball, arch_tarball);

    symlink(&tool,&ver)?;
    println!("Installed {} @ {}", tool, ver);

    Ok(())
}

fn pick(input : String) -> Result<(), Box<dyn std::error::Error>> {
    let (tool, ver) = input.split_once("@")
        .ok_or("Invalid format: use 'tool@version'")?;

    let tool_ver_path = format!("{}/.scarecrow/versions/{}/{}", std::env::var("HOME")?, 
        &tool, &ver);
    if !fs::exists(&tool_ver_path)? {
        return Err("Version not installed. Run 'crow guard' to install it first.".into());
    }

    symlink(tool, ver)?;
    println!("Switched to {}@{}", tool, ver);
    
    Ok(())
}

fn scare(input : String) -> Result<(), Box<dyn std::error::Error>> {
    let (tool, ver) = input.split_once("@")
        .ok_or("Invalid format: use 'tool@version'")?;

    let tool_ver_path = format!("{}/.scarecrow/versions/{}/{}", std::env::var("HOME")?, 
        &tool, &ver);
    let symlink_path = format!("{}/.scarecrow/bin/{}", std::env::var("HOME")?, tool);
    if !fs::exists(&tool_ver_path)? {
        return Err("Version does not exist".into());
    }
    let is_active = fs::read_link(&symlink_path)
        .ok()
        .map(|target| target.to_string_lossy().contains(ver))
        .unwrap_or(false);
    if is_active {
        return Err("Cannot remove active version. Run 'crow pick' to switch to a different version first.".into());
    } else {
        fs::remove_dir_all(&tool_ver_path)?;
        Ok(())
    }

}

fn list_all_ver() -> Result<(), Box<dyn std::error::Error>> {
    let ver_dir = format!("{}/.scarecrow/versions", std::env::var("HOME")?);
    let path = Path::new(&ver_dir);

    if !path.exists() {
        println!("🐦‍⬛ No versions installed yet. To install a version, run 'crow guard'.");
        return Ok(());
    }

    println!("🐦‍⬛ Installed versions: ");

    for tool_entry in fs::read_dir(path)? { //iterate through all tools in directory
        let tool_entry = tool_entry?; //halt loop and throw error if erroneous value is unwrapped
        let tool_path = tool_entry.path(); //actual tool file path
        let tool_name = tool_entry.file_name(); //tool name
        let tool_name = tool_name.to_string_lossy(); //convert to Rust string

        if !tool_path.is_dir() {  //skip if not dir
            continue;
        }

        //read ver under this tool
        let mut ver = Vec::new(); //new mutable variable of vector array type
        for ver_entry in fs::read_dir(&tool_path)? { //each tool has a different version with its own info
            let ver_entry = ver_entry?;
            let ver_path = ver_entry.path();
            let ver_name = ver_entry.file_name();

            if ver_path.is_dir() { //if the version is a directory
                ver.push(ver_name.to_string_lossy().to_string()); //add it to the vector
            }
        }

        //print tool and its ver
        if !ver.is_empty() {
            ver.sort();
            println!("  {}: {}", tool_name, ver.join(", "));
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
match cli.command {
        Commands::Guard { tool_ver } => {
            if let Err(e) = guard(tool_ver) {
                eprintln!("🐦‍⬛ Error: {}", e); 
            }
        }
        Commands::Pick { tool_ver } => {
            //println!("🐦‍⬛ Picking environment {}", tool_ver);
            if let Err(e) = pick(tool_ver) {
                eprintln!("🐦‍⬛ Error: {}", e);
            }
        }
        Commands::Scare { tool_ver } => {
            if let Err(e) = scare(tool_ver) {
                eprintln!("🐦‍⬛ Error: {}", e);
            }
        }
        Commands::All => {
            if let Err(e) = list_all_ver() {
                eprintln!("🐦‍⬛ Error: {}", e);
            }
        }
    }
}
