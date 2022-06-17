use serde_derive::{Deserialize, Serialize};
use reqwest::{Error, Url};

#[derive(Serialize, Deserialize, Debug)]
struct Route {
  alerts: Vec<String>,
  authority: String,
  directions: Vec<String>,
  id: String,
  name: String,
  shortName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stop {
  id: String,
  name: String,
  number: String
}

#[derive(Serialize, Deserialize, Debug)]
struct StopsResponse {
  route: Route,
  stops: Vec<Stop>,
}

impl StopsResponse {
  fn create_empty() -> StopsResponse {
    StopsResponse {
      route: Route::create_empty(),
      stops: vec![],
    }
  }
}

impl Route {
  fn create_empty() -> Route {
    Route {
      alerts: vec![],
      authority: String::new(),
      directions: vec![],
      id: String::new(),
      name: String::new(),
      shortName: String::new(),
    }
  }

  async fn get_stops(&self) -> Result<StopsResponse, Error> {
    let url_str = format!(
      "http://ttss.mpk.krakow.pl/internetservice/services/routeInfo/routeStops?routeId={}",
      self.id
    );
    let url = match Url::parse(&*url_str) {
      Ok(url) => { url }
      Err(e) => {
        println!("ParseError {:?}, {:?} ", url_str, e);
        return Ok(StopsResponse::create_empty());
      }
    };
    reqwest::get(url).await?.json::<StopsResponse>().await
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct RoutesResponse {
  routes: Vec<Route>
}

impl RoutesResponse {
  async fn get() -> Result<RoutesResponse, Error> {
    let request_url = "http://ttss.mpk.krakow.pl/internetservice/services/routeInfo/route";
    let response = reqwest::get(request_url).await?;
    response.json::<RoutesResponse>().await
  }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  println!("mleko");

  for route in RoutesResponse::get().await?.routes {
    println!("{:?}", route);
    let stops =  route.get_stops().await?;
    println!("{:?}", stops);
    break;
  }

  Ok(())
}
