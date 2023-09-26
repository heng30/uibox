use crate::config;
use reqwest::{Client, Proxy, Result};
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClientType {
    Local = -1,
    OpenAI = 0,
}

pub fn client(cln_type: ClientType, timeout: u64) -> Result<Client> {
    let conf = config::socks5();
    Ok(if cln_type == ClientType::OpenAI && conf.openai {
        let proxy = Proxy::all(format!("socks5://{}:{}", conf.url, conf.port))?;
        Client::builder()
            .proxy(proxy)
            .timeout(Duration::from_secs(timeout))
            .build()?
    } else {
        Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()?
    })
}
