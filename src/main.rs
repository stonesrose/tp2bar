    // TODO: Add variable support: env:SWIFTBAR_PLUGIN_DATA_PATH = /Users/mluich/Library/Application Support/SwiftBar/Plugins/tp2bar.1m.rs
    // TODO: Add support for all on & off
    // TODO: Do we want the about/info/other displayed?
    // TODO: Node reload/reset??? Docs are unclear.
    // TODO: How to handle Script Metadata https://github.com/swiftbar/SwiftBar?tab=readme-ov-file#script-metadata
    //       https://github.com/swiftbar/SwiftBar?tab=readme-ov-file#metadata-for-binary-plugins
    // TODO: Add hostname to config

use std::fs;
use reqwest;    
use serde_json;
use serde_derive::Deserialize;
use serde_derive::Serialize;

/// Tp2Root is the top level json
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tp2Root {
    pub response: Vec<Tp2Response>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tp2Response {
    pub result: Vec<Tp2Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tp2Result {
    pub node1: String,
    pub node2: String,
    pub node3: String,
    pub node4: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Tp2Config {
    passwd: String,
}

impl Tp2Config {
    fn new () -> Tp2Config {
        let mut config: Tp2Config = Tp2Config { passwd: "cm9vdDp0dXJpbmc=".to_string()};
        config = Tp2Config::get(config);
        return config;
    }

    fn get (config: Tp2Config) -> Tp2Config{
        let data = fs::read_to_string("./tp2bar.json");
        // Check
        if data.is_ok() {

            let config: Tp2Config = serde_json::from_str(data.unwrap().as_str()).unwrap();
            return config;
        } else {
            return config;
        }
    }
}

fn set_node_state(node_id: String, requested_node_state: String){

    let url = format!("https://turingpi.local/api/bmc?opt=set&type=power&node{}={}", node_id, requested_node_state );
    let status = reqwest::blocking::Client::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .unwrap()
    .get(url)
    .header("Authorization", "Basic cm9vdDp0dXJpbmc=")
    .send();
   
   println!("{:?}", status)
}



fn main() {

    let config = Tp2Config::new();
    let node_id = std::env::args().nth(1); 
    let requested_state = std::env::args().nth(2); 
    let mut header_string: String= String::new();
    let mut menu_string: String= String::new();


    //Set the requested state.
    if node_id.is_some() && requested_state.is_some() {
        set_node_state(node_id.unwrap(), requested_state.unwrap());
    }
    
    //get the current state
    //curl 'https://turingpi.local/api/bmc/authenticate' -X POST -d '{"username":"root", "password":"turing"}' -k
    // cm9vdDp0dXJpbmc=
   
   let auth_value = format!("Basic {}", config.passwd);
   let status = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("https://turingpi.local/api/bmc?opt=get&type=power")
        .header("Authorization", auth_value)
        .send();

    // Strip the result down to the actual node status
    let current_status: Tp2Result = status.unwrap().json::<Tp2Root>().unwrap().response[0].result[0].clone();
    //println!("Node1: {:?}", current_status.node1);


    if current_status.node1 == "0" {
        header_string = ":bolt.circle:".to_string();
        menu_string = "Node 1\n-- Start| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=1 param2=1 terminal=false refresh=true\n".to_string();
    } else if current_status.node1 == "1" {
        header_string = ":bolt.circle.fill:".to_string();
        menu_string = "Node 1\n-- Stop| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=1 param2=0 terminal=false refresh=true\n".to_string();
    } else {
        header_string = ":exclamationmark.warninglight:\n".to_string();
        menu_string = "???? Node 1".to_string();
    }
    // I could do this as a loop but for 4 knowns ..... nah
    // loop would require creating an iter for Tp2Result
    // would be worthwhile for multiple setups?
    if current_status.node2 == "0" {
        header_string = header_string + ":bolt.circle:";
        menu_string = menu_string + "Node 2\n-- Start| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=2 param2=1 terminal=false refresh=true\n";
    } else if current_status.node2 == "1" {
        header_string = header_string + ":bolt.circle.fill:";        
        menu_string = menu_string + "Node 2\n-- Stop| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=2 param2=0 terminal=false refresh=true\n";
    } else {
        header_string = header_string + ":exclamationmark.warninglight:";
        menu_string = menu_string + "???? Node 2\n";
    }
   
    if current_status.node3 == "0" { 
        header_string = header_string + ":bolt.circle:";
        menu_string = menu_string + "Node 3\n-- Start| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=3 param2=1 terminal=false refresh=true\n";

    } else if current_status.node3 == "1" {
        header_string = header_string + ":bolt.circle.fill:";
        menu_string = menu_string + "Node 3\n-- Stop| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=3 param2=0 terminal=false refresh=true\n";

    } else {
        header_string = header_string + ":exclamationmark.warninglight:";
        menu_string = menu_string + "???? Node 3\n";
    }

    if current_status.node4 == "0" {
        header_string = header_string + ":bolt.circle:";
        menu_string = menu_string + "Node 4\n-- Start| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=4 param2=1 terminal=false refresh=true\n";
    } else if current_status.node4 == "1" {
        header_string = header_string + ":bolt.circle.fill:";
        menu_string = menu_string + "Node 4\n-- Stop| bash='/Users/mluich/swiftbar/tp2bar.*.rs' param1=4 param2=0 terminal=false refresh=true\n";
    } else {
        header_string = header_string + ":exclamationmark.warninglight:";
        menu_string = menu_string + "???? Node 4\n";
    }

    println!("{}| dropdown=true", header_string);
    println!("---");
    println!("{}", menu_string);
    println!("---");

    // let key = "SWIFTBAR_PLUGIN_DATA_PATH";
    // match env::var(key) {
    //     Ok(val) => println!("{key}: {val:?}"),
    //     Err(e) => println!("couldn't interpret {key}: {e}"),
    // }
    // for (n,v) in env::vars() {
    //     println!("{}: {}", n,v);
    // }

}   
