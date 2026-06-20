use std::fs;
use std::path::Path;
use std::os::unix::fs as unix_fs;
use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use reqwest;
use tar::Archive;

#[derive(Parser)] //automatically generate code to parse command line arguments, Rust writes boilerplate for me :p
#[command(name = "crow")]
#[command(about = "scarecrow is a CLI tool for managing development environments 🐦‍⬛", long_about = None)]
struct Cli { //defines my command line interface, the struct will hold the parsed arguments
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)] //automatically generate code to parse subcommands
enum Commands {
    Fetch { //install and switch to env
        tool_ver: String, //ver of tool to use e.g. node 18.16.0
    },
    Perch { //change env
        tool_ver: String, //ver of tool to use e.g. node 18.16.0
    },
    Drop { //remove env
        tool_ver: String, //ver of tool to remove e.g. node 18.16.0
    },
    All, //list all envs
}

//helpers
fn build_url(tool: &str, version: &str, os: &str, arch: &str) -> Result<String, Box<dyn std::error::Error>> {
    let os_mapped = map_os(tool, os);
    let arch_mapped = map_arch(tool, arch);
    
    match tool {
        "node" => Ok(format!("https://nodejs.org/dist/v{}/node-v{}-{}-{}.tar.gz", version, version, os_mapped, arch_mapped)),
        "go" => Ok(format!("https://go.dev/dl/go{}.{}-{}.tar.gz", version, os_mapped, arch_mapped)),
        _ => Err("Unsupported tool".into())
    }
}

fn map_os(tool: &str, os: &str) -> &'static str {
    match tool {
        "node" => match os {
            "macos" => "darwin",
            "windows" => "win",
            "linux" => "linux",
            _ => "unknown",
        },
        "go" => match os {
            "macos" => "darwin",
            "linux" => "linux",
            _ => "unknown",
        },
        _ => "unknown",
    }
}

fn map_arch(tool: &str, arch: &str) -> &'static str {
    match tool {
        "node" => match arch {
            "aarch64" => "arm64",
            "x86_64" => "x64",
            _ => "unknown",
        },
        "go" => match arch {
            "aarch64" => "arm64",
            "x86_64" => "amd64",
            _ => "unknown",
        },
        _ => "unknown",
    }
}
fn symlink(tool : &str, version : &str, os: &str, arch: &str) -> Result<(), Box<dyn std::error::Error>> {
    //creating symlink directory, for the tool the user is using scarecrow to install
    let os_mapped = map_os(tool, os);
    let arch_mapped = map_arch(tool, arch);
    let symlink_dir = format!("{}/.scarecrow/bin", std::env::var("HOME")?);
    fs::create_dir_all(&symlink_dir)?; //only created if path does not exist

    let symlink_path = Path::new(&symlink_dir).join(tool); //where the symlink file will be
    let target = match tool {
        "go" => format!("../versions/{}/{}/go/bin/{}", tool, version, tool),
        _ => format!("../versions/{}/{}/{}-v{}-{}-{}/bin/{}", tool, version, tool, version, os_mapped, arch_mapped, tool),
    };  
    if symlink_path.exists() { //remove old symlinks
        fs::remove_file(&symlink_path)?;
    }

    unix_fs::symlink(&target, &symlink_path)?; //create the symlink
    Ok(())
}

fn download(tar_url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.get(tar_url)
        .send()?;

    println!("{}",tar_url);
    
    if !res.status().is_success() {
        return Err(format!("Download failed with status: {}", res.status()).into());
    }
    
    Ok(res.bytes()?.to_vec())
}

fn decomp_install(bytes: &[u8], path: &str, format: &str)-> Result<(), Box<dyn std::error::Error>> {
   match format {
        "tar.gz" => {
            let decoder = GzDecoder::new(bytes.as_ref());
            let mut archive = Archive::new(decoder);
            archive.unpack(path)?;
        }
        _ => {
            return Err("Unsupported format".into());
        }
    }
    Ok(())
}


//crow command functions
fn fetch(input : String) -> Result<(), Box<dyn std::error::Error>> {
    //download link example: https://nodejs.org/dist/v24.17.0/node-v24.17.0-darwin-arm64.tar.gz
    //download link structure: https://nodejs.org/dist/v{version}}/node-v{version}-{os}-{arch}.tar.gz  

    let (tool, ver) = input.split_once("@")
        .ok_or("Invalid format: use 'tool@version'")?;
    
    let base_path = format!("{}/.scarecrow/versions", std::env::var("HOME")?);
    let path = Path::new(&base_path); //convert to Path type

    let tool_dir = path.join(tool);
    let ver_dir = tool_dir.join(ver); 

    fs::create_dir_all(&ver_dir)?; //creates the tool directory by joining path, tool name and version

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    let tarball = build_url(tool, ver, os, arch)?;
    let bytes = download(&tarball)?;
    decomp_install(&bytes, &ver_dir.to_string_lossy(), "tar.gz")?;

    symlink(&tool, &ver, os, arch)?;
    println!("🐦‍⬛ Installed {} @ {}", tool, ver);

    Ok(())
}

fn perch(input : String) -> Result<(), Box<dyn std::error::Error>> {
    let (tool, ver) = input.split_once("@")
        .ok_or("Invalid format: use 'tool@version'")?;

    let tool_ver_path = format!("{}/.scarecrow/versions/{}/{}", std::env::var("HOME")?, 
        &tool, &ver);
    if !fs::exists(&tool_ver_path)? {
        return Err("Version not installed. Run 'crow fetch' to install it first.".into());
    }

    let arch = std::env::consts::ARCH;
    let os = std::env::consts::OS;

    symlink(&tool, &ver, os, arch)?;
    println!("Switched to {}@{}", &tool, &ver);
    
    Ok(())
}

fn crow_drop(input : String) -> Result<(), Box<dyn std::error::Error>> {
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
        return Err("Cannot remove active version. Run 'crow perch' to switch to a different version first.".into());
    } else {
        fs::remove_dir_all(&tool_ver_path)?;
        Ok(())
    }

}

fn list_all_ver() -> Result<(), Box<dyn std::error::Error>> {
    let ver_dir = format!("{}/.scarecrow/versions", std::env::var("HOME")?);
    let path = Path::new(&ver_dir);

    if !path.exists() {
        println!("🐦‍⬛ No versions installed yet. To install a version, run 'crow fetch'.");
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
        Commands::Fetch { tool_ver } => {
            if let Err(e) = fetch(tool_ver) {
                eprintln!("🐦‍⬛ Error: {}", e); 
            }
        }
        Commands::Perch { tool_ver } => {
            //println!("🐦‍⬛ perching environment {}", tool_ver);
            if let Err(e) = perch(tool_ver) {
                eprintln!("🐦‍⬛ Error: {}", e);
            }
        }
        Commands::Drop { tool_ver } => {
            if let Err(e) = crow_drop(tool_ver) {
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
