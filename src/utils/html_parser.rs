use scraper::{Html, Selector};

pub fn get_courses_keys(html : String) -> Vec<String>{
    let mut keys = Vec::new();
    let fragment = Html::parse_fragment(html.as_str());
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let ul = fragment.select(&ul_selector).skip(6).next().unwrap();
    for element in ul.select(&li_selector) {
        keys.push( String::from(element.value().attr("data-key").unwrap()));
    }
    return keys;
}

pub fn get_course_name(html : String) -> String{
    let fragment = Html::parse_fragment(html.as_str());
    let title_selector = Selector::parse("title").unwrap();

    let title = fragment.select(&title_selector).next().unwrap();
    let course_title = &title.inner_html()[7..];
    return String::from(course_title);
}

pub fn get_resources_page_url(html : String) -> String {
    let fragment = Html::parse_fragment(html.as_str());
    let resources_selector = Selector::parse(r#"li[aria-label="Materiali"]"#).unwrap();
    let link_selector = Selector::parse(r#"a[class="course-card-link"]"#).unwrap();
    let res_index = fragment.select(&resources_selector).next();
    if res_index == None {
        return String::from("CCS");
    }
    let index = res_index.unwrap().select(&link_selector).next().expect("msg").value().attr("href").unwrap();
    return String::from(index);
}

pub fn get_files_url(html : String) -> (Vec<(String,String)>,Vec<(String,String)>) {
    let mut course_files = Vec::new();
    let mut course_dirs = Vec::new();

    let fragment = Html::parse_fragment(html.as_str());

    let files_selector = match Selector::parse(r#"a[class="aalink"]"#) {
        Ok(selector) => selector,
        Err(_error) => return (Vec::new(),Vec::new())
    };

    let names_selector = match Selector::parse(r#"span[class="instancename"]"#) {
        Ok(selector) => selector,
        Err(_error) => return (Vec::new(),Vec::new())
    };

    let names = fragment.select(&names_selector);
    let mut files = fragment.select(&files_selector);
    for name in names {
        let pos_name = name.inner_html();
        let file = files.next().expect("msg");
        if pos_name.contains("File") {
            let name_size = pos_name.find("<span").expect("msg");
            let file_name : String = pos_name.chars().take(name_size).collect();
            let url = file.value().attr("href").expect("msg");
            course_files.push((file_name,String::from(url)));
        } 
        else if pos_name.contains("Cartella") {
            let name_size = pos_name.find("<span").expect("msg");
            let dir_name : String = pos_name.chars().take(name_size).collect();
            let url = file.value().attr("href").expect("msg");
            course_dirs.push((dir_name,String::from(url)));
        }
    }
    return (course_files, course_dirs);
}
