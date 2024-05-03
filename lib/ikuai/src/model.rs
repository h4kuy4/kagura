use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EtherInfoValue {
    pub driver: String,
    #[serde(rename = "type")]
    pub t: String,
    pub mac: String,
    pub link: i32,
    pub speed: i32,
    pub duplex: i32,
    pub model: String,
    pub interface: String,
    pub lock: i32,
}

#[derive(Debug, Deserialize)]
pub struct SnapshootLan {
    pub id: i32,
    pub comment: String,
    pub interface: String,
    pub bandmode: i32,
    pub linkmode: i32,
    pub mac: String,
    pub member: Vec<String>,
    pub ip_addr: String,
    pub netmask: String,
}

#[derive(Debug, Deserialize)]
pub struct SnapshootWan {
    pub id: i32,
    pub comment: String,
    pub interface: String,
    pub mac: String,
    pub member: Vec<String>,
    pub bandmode: i32,
    pub default_route: i32,
    pub internet: i32,
    pub ip_addr: String,
    pub netmask: String,
    pub gateway: String,
    pub dns1: String,
    pub dns2: String,
    pub count_static: i32,
    pub count_dhcp: i32,
    pub count_pppoe: i32,
    pub count_check_fail: i32,
    pub updatetime: i32,
    pub check_res: i32,
    pub errmsg: String,
    pub power: String,
    pub isp: String,
    pub imei: String,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub ether_info: HashMap<String, EtherInfoValue>,
    pub snapshoot_lan: Vec<SnapshootLan>,
    pub snapshoot_wan: Vec<SnapshootWan>,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "Result")]
    pub result: i32,
    #[serde(rename = "ErrMsg")]
    pub err_msg: String,
    #[serde(rename = "Data")]
    pub data: Data,
}
