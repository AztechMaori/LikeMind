
use axum::Extension;




use super::authguard::Passed;


pub async fn check(Extension(stuff): Extension<Passed>){
 println!("the authguard middleware worked here is the userid: {}. Here is whether the request should resend the cookie {}", stuff.userid, stuff.send);
}