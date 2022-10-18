use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct User{
    username:String,
    password:String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/login").post(check_credentials);
    println!("Something happened 1");
    app.listen("127.0.0.1:7878").await?;
    Ok(())
}

async fn check_credentials(mut req:Request<()>) -> tide::Result {
    println!("Something happened 2");
    println!("given: {}",req.body_string().await?);
    let User { username, password } = req.body_json().await?;
    println!("Something happened 3");
    println!("username:{},password:{}",username,password);
    Ok(("Given Username").into())
}