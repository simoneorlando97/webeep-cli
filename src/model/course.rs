#[derive(Debug)]
#[derive(Clone)]
pub struct Course {
    pub id : String,
    pub name : String,
    pub resource_page_url : String,
    pub resources_page : Option<String>,
    pub files : Vec<(String,String)>,
    pub dirs : Vec<(String,String)>,
    pub dirs_with_files : Vec<(String,Vec<(String,String)>)>
}

impl Course {
    pub fn new(id : &str, name : &str, index : &str) -> Course {
        return Course {
            id:String::from(id),
            name:String::from(name), 
            resource_page_url:String::from(index),
            resources_page:None,
            files: Vec::new(),
            dirs: Vec::new(),
            dirs_with_files: Vec::new()
        };
    }

    pub fn set_resources_page(&mut self,page : String) {
        self.resources_page = Some(page);
    }
}
