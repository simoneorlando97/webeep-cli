use reqwest::blocking::*;
use scraper::{Html, Selector};

pub struct HttpClient {
    client : Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client_builder = ClientBuilder::new().cookie_store(true);
        let client = client_builder.build().expect("Failed to build HTTP client!");
        return HttpClient{client};
    }

    pub fn get_redirect(&mut self, url : String) -> Response {
        let res = self.client.get(url).send().expect("Could not get the requested url!");
        return res;
    }

    pub fn get_action_from_html(&mut self,response : Response) -> String{
        let html = response.text().expect("Error while parsing the html!");
        let start_index = html.find(" action=").expect("Error while parsing the html!") + 9;
        let end_index = html.find("\" class=\"jaf-form jaf-mobile-form\"><button").expect("Error while parsing the html!");
        let action = &html[start_index..end_index];
        return String::from(action);
    }

    pub fn get_hidden_param_from_html(&mut self,response : Response) -> (String,String) {
        let html = response.text().expect("Error while parsing the html!");
        let fragment = Html::parse_fragment(html.as_str());

        let param_selector = Selector::parse("input").unwrap();
        let mut selector = fragment.select(&param_selector);
        let first_param = selector.next().expect("Please check your credentials or if you need to update the password on online services.").value().attr("value").expect("Please check your credentials or if you need to update the password on online services!");
        let second_param = selector.next().expect("Please check your credentials or if you need to update the password on online services.").value().attr("value").expect("Please check your credentials or if you need to update the password on online services!");
        return (String::from(first_param), String::from(second_param));
    }

    pub fn post_req(&mut self, url : String, id : &str, pass : &str) -> Response {
        let params = [("reg", ""),("login",id),("password",pass),("evn_conferma",""),("RESTA_CONNESSO","-2147483648")];
        let res = self.client.post(url).form(&params).send().expect("Failed to execute post request!");
        return res;
    }

    pub fn hidden_post_req(&mut self, url : String, first_param : String, second_param : String) -> Response {
        let params = [("RelayState", first_param.as_str()),("SAMLResponse",second_param.as_str())];
        let res = self.client.post(url).form(&params).send().expect("Failed to execute hidden post request");
        return res;
    }
}