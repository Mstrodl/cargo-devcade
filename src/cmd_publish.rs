use crate::Args;

pub fn publish(_args: &Args) {
  let url = "https://devcade.csh.rit.edu/upload";
  log::info!("I can't take you any further!");
  log::info!("Navigate to {url} in a browser to complete the upload.");
  if let Err(err) = webbrowser::open(url) {
    log::warn!("Couldn't open your default browser: {:?}", err);
  }
}
