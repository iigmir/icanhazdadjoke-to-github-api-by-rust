use async_std::task;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>
{
    #[derive(Deserialize, Serialize)]
    struct Ip {
        ip: String
    };

    task::block_on( async {
        let uri = "https://httpbin.org/post";
        let data = &Ip { ip: "129.0.0.1".into() };
        let res = surf::post(uri).body_json(data)?.await?;
        assert_eq!(res.status(), 200);

        let uri = "https://api.ipify.org?format=json";
        let Ip { ip } = surf::get(uri).recv_json().await?;
        assert!(ip.len() > 10);
        Ok(())
    });
}