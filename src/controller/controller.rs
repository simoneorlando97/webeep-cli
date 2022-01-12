use crate::utils;
use std::path::Path;
use reqwest::blocking::Response;
use scraper::{Html, Selector};
use crate::utils::{HttpClient, get_course_name, get_courses_keys, get_files_url, get_resources_page_url};
use crate::model::Course;
use std::fs::File;
use std::io;


pub struct Controller {
    pub client : HttpClient,
    pub id  : String,
    pub pass : String,
    pub otp : String,
    pub courses : Vec<Course>,
}

impl Controller {

    pub fn new(id : &str, pass : &str, otp : &str) -> Controller {
        return Controller{
            client : HttpClient::new(),
            id : String::from(id),
            pass : String::from(pass),
            otp : String::from(otp),
            courses : Vec::new()
        };
    }

    pub fn start(&mut self) {
        let id = self.id.clone();
        let pass = self.pass.clone();
        let otp = self.otp.clone();
        let resp = self.login(id.as_str(),pass.as_str(),otp.as_str());
        println!("Fetching courses' informations, this may take a while.");
        let courses_names = self.get_courses_names(resp);
        for course in courses_names {
            self.courses.push(Course::new(course.1.as_str(),course.0.as_str(),course.2.as_str()));
        }
        self.get_resources_page();
        self.get_files();
        self.get_dirs();
        println!("Everything is ready!");
    }

    fn login(&mut self, id : &str, pass : &str, otp : &str) -> Response{
        //ID e PASSWORD + CONFERMA
        let first_response = self.client.get_redirect(String::from(utils::WE_BEEP_URL));
        println!("Connecting to WeBeep...");
        let action = self.client.get_action_from_html(first_response);
        let mut final_action = String::from(utils::AUNICA_URL);
        println!("Redirecting to Aunica.");
        final_action.push_str(action.as_str());
        let post_response = self.client.post_req(final_action.clone(), id, pass);
        println!("Sending credentials.");

        
        //OTP + CONTINUA
        let action_otp = self.client.get_action_from_html_otp(post_response);
        let mut final_action_otp = String::from(utils::AUNICA_URL);
        final_action_otp.push_str(action_otp.as_str());

        let otp_response = self.client.otp_post_req(final_action_otp.clone(), &otp);

        println!("Sending OTP code.");

        //SAML2
        let hidden_action_parameters = self.client.get_hidden_param_from_html(otp_response);
        println!("Extracting hidden parameters.");

        let landing_page = self.client.hidden_post_req(String::from(utils::HIDDEN_POST_URL), hidden_action_parameters.0, hidden_action_parameters.1);
        println!("Successfully login!");
        return landing_page;
    }


    fn get_courses_names(&mut self, response : Response) -> Vec<(String,String,String)> {
        let keys = get_courses_keys(response.text().expect(""));
        let mut courses_names = Vec::new();
        for key in keys {
            let mut arg = String::from(utils::WE_BEEP_COURSE_URL);
            arg.push_str("?id=");
            arg.push_str(key.as_str());
            let response = self.client.get_redirect(arg);
            let course_page = response.text().expect("");
            let course_name = get_course_name(course_page.clone());
            let index = get_resources_page_url(course_page.clone());
            if !index.eq("CCS") {
                courses_names.push((course_name,key,index)); 
            }
        }
        return courses_names;
    }

    fn get_resources_page(&mut self) {
        for course in &mut self.courses {
            let resources_page = self.client.get_redirect(course.resource_page_url.clone());
            course.set_resources_page(resources_page.text().expect("msg"));
        }
    }

    fn get_files(&mut self) {
        let mut files ;
        for course in &mut self.courses {
            files = get_files_url(course.resources_page.clone().expect("msg"));
            course.files = files.0;
            course.dirs = files.1;
        }
    }

    pub fn download_file(&mut self, file : (String,String), dest : String) {
        let mut response = self.client.get_redirect(file.1);
        let dest_dir = dest.clone();
        let path = Path::new(dest_dir.as_str());
        let new_pat = path.join(file.0.as_str());
        
        let file_fin = File::create(new_pat);
        if file_fin.is_err() {
            println!("Incorrect destination path");
            return;
        }
        else {
            let mut corr_file = file_fin.unwrap();
            io::copy(&mut response, &mut corr_file).expect("Impossible to save the file!");
        }
        println!("{} downloaded", file.0);
    }

    pub fn get_dirs(&mut self) {
        for course in &mut self.courses {
            let mut files = Vec::new();
            for dir in &course.dirs {
                let dir_url = dir.1.clone();
                let dir_page = self.client.get_redirect(dir_url);
                let fragment = Html::parse_fragment(dir_page.text().unwrap().as_str());
                let dir_selector = Selector::parse(r#"span[class="fp-filename-icon"]"#).unwrap();
                let link_selector = Selector::parse("a").unwrap();
                let name_selector = Selector::parse(r#"span[class="fp-filename"]"#).unwrap();
                let dirs = fragment.select(&dir_selector);
                for d in dirs {
                    let link = d.select(&link_selector).next().unwrap().value().attr("href").unwrap();
                    let name = d.select(&name_selector).next().unwrap().inner_html();
                    files.push((name,String::from(link)));
                }
                course.dirs_with_files.push((dir.0.clone(),files.clone()));
            }
        }
    }
}