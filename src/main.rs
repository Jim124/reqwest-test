use reqwest::blocking::{Client,ClientBuilder};
use reqwest::redirect::Policy;

fn main() {
    println!("Hello, world!");
    let http_client = Client::new();
    let http_result = http_client.get("http://google.com").send();
    if http_result.is_ok(){
        println!("{:#?}",http_result.ok().unwrap());
     } else{
        println!("{:#?}",http_result.err() );
     }

   let http_post_result = Client::new().post("http://google.com").
   body("{\"first_name\":\"Trevor\"}").header("agent-agent", "jim du").send();
   println!("{:#?}",http_post_result.ok() );

   //redirect limited
    let policy = Policy::limited(10);
    let client_redirect = ClientBuilder::new().redirect(policy).build().ok().unwrap();
    let http_result_2 = client_redirect.get("http://localhost:3000/weather").send().ok().unwrap();
    println!("{http_result_2:#?}");

}
