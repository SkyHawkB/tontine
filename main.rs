use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};
use rand::prelude::*;
use math::round;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let mut caps = DesiredCapabilities::firefox();
     caps.set_headless()?;
     
     let driver = WebDriver::new("http://localhost:4444", &caps).await?;

     driver.get("https://tontine.cash").await?;
     sleep(Duration::from_millis(2313)).await;

     let activate_btn = driver.find_element(By::Css(".btn-small")).await?;
     activate_btn.click().await?;

     let email = "EMAIL_HERE";
     let password = "PASSWORD_HERE";

     let email_field = driver.find_element(By::Css("input[placeholder=email]")).await?;
     let password_field = driver.find_element(By::Css("input[placeholder=password]")).await?;
     let mut rng = rand::thread_rng();
     let delay = 100_f64;
     let vary = 40_f64;

     for c in email.chars() {
         email_field.send_keys(String::from(c)).await?;
         sleep(Duration::from_millis(round::floor(delay+rng.gen::<f64>()*vary, 0) as u64)).await;
     }

     for c in password.chars() {     
           password_field.send_keys(String::from(c)).await?;
           sleep(Duration::from_millis(round::floor(delay+rng.gen::<f64>()*vary, 0) as u64)).await;     
     }

     sleep(Duration::from_millis(69)).await;

     let login_btn = driver.find_element(By::Css(".login-btn")).await?;
     login_btn.click().await?;

     sleep(Duration::from_millis(559)).await;

     let alive_btn = driver.find_element(By::Css(".banner-btn")).await?;
     alive_btn.click().await?;

     sleep(Duration::from_millis(6000)).await;
     
     driver.quit().await?;
     Ok(())
}
