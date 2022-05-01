
use reqwest;
use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct Attributes {
    Name: String,
    Hash: String,
    Size: String,
}

fn main() {
    let server = "127.0.0.1";
    let port = 5001;


    let url_string = format!("http://{}:{}?pin=true", server, port);
    let mut url = reqwest::Url::parse(&url_string).unwrap();
    url.set_path("api/v0/add");
    

    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
    .multipart(reqwest::blocking::multipart::Form::new()
                .part("name", reqwest::blocking::multipart::Part::text("data"))
                ).send();

   match res {
       Ok(r) => {
           let attr = r.json::<Attributes>().unwrap();
           println!("name: {}", attr.Name);
           println!("hash: {}", attr.Hash);
           println!("size: {}", attr.Size);
       },
       Err(e) => {
           println!("{:?}", e);
       }
   }

   let url_string2 = format!("http://{}:{}", server, port);
   let mut url2 = reqwest::Url::parse(&url_string2).unwrap();
   url2.set_path("api/v0/cat");

   let res_cat = client.post(url2)
        .query(&[("arg", "QmSBpYfb6AM4ioR4qDeqP5wzbVP5z85fGsKUxvHSkvKTqg")])
        .send();
    match res_cat {
        Ok(r) => {
            println!("{:?}", r.text());
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }

}