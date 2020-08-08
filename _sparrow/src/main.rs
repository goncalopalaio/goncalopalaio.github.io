use chrono::NaiveDate;
use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

const PAGE_PREFIX: &'static str = "page-";
const POST_PREFIX: &'static str = "post-";
const UNLISTED_POST_PREFIX: &'static str = "unlisted-post-";

const TITLE_CONTENT_PREFIX: &'static str = "title = ";
const DATE_CONTENT_PREFIX: &'static str = "date = ";

const TITLE_REPLACEMENT: &'static str = "<!-- #insert-title -->";
const CONTENT_REPLACEMENT: &'static str = "<!-- #insert-content -->";

#[derive(Debug)]
struct Post {
    name: String,
    title: String,
    date_str: String,
    date: NaiveDate,
    author: String,
    content: String,
}

fn get_comrak_options() -> ComrakOptions {
    // using _unsafe to insert allow html in the md files
    // used to set the size of images in the showcase page
    // and some random </br> to fix the look of the page
    return ComrakOptions {
        unsafe_: true,
        ext_strikethrough: true,
        ..ComrakOptions::default()
    };
}

fn read_posts(list: &Vec<PathBuf>) -> std::vec::Vec<Post> {
    let mut posts = Vec::<Post>::new();

    for file in list {
        let file_contents = fs::read_to_string(file).unwrap();
        let full_content: Vec<_> = file_contents.lines().collect();

        let name = md_path_to_name(&file);

        let mut content = String::new();
        let mut title = uppercase_first_letter(&name);
        let mut date_str = String::new();

        for line in full_content {
            if line.starts_with(TITLE_CONTENT_PREFIX) {
                title = line.replace(TITLE_CONTENT_PREFIX, "").to_string();
            } else if line.starts_with(DATE_CONTENT_PREFIX) {
                date_str = line.replace(DATE_CONTENT_PREFIX, "").to_string();
            } else {
                content.push_str(line);
                content.push_str("\n")
            }
        }

        let date = NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").unwrap();

        let post = Post {
            name: name,
            title: title,
            date_str: date_str,
            date: date,
            content: content,
            author: "Gonçalo".to_string(),
        };

        posts.push(post);
    }

    posts.sort_by(|a, b| a.date.partial_cmp(&b.date).unwrap());
    posts.reverse();

    for p in &posts {
        println!(
            "Post: {} | {} | {} | {}",
            p.name, p.title, p.date_str, p.date
        );
    }

    return posts;
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    let mut capitalized = String::new();

    match c.next() {
        None => {
            capitalized.push_str(s);
        }
        Some(f) => capitalized.push_str(&f.to_uppercase().chain(c).collect::<String>()),
    };

    return capitalized;
}

fn md_path_to_name(md_path: &PathBuf) -> String {
    let name = md_path.file_stem().unwrap().to_str().unwrap();
    
    let mut name = name.replace(PAGE_PREFIX, "");
	name = name.replace(POST_PREFIX, "");
	name = name.replace(UNLISTED_POST_PREFIX, "");

    return name;
}

fn get_md_files(prefix: &str) -> Vec<PathBuf> {
    return fs::read_dir("contents/")
        .unwrap()
        .into_iter()
        .filter(|f| f.is_ok())
        .filter(|f| {
            let file_name = f.as_ref().unwrap().file_name();
            let name = file_name.to_str().unwrap();
            name.ends_with(".md") && name.starts_with(&prefix)
        })
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
}

fn create_md_header(title: &str, pages: &Vec<PathBuf>) -> String {
    let mut md_content = String::new();

    md_content.push_str(&format!("#### [{}](index.html)", title));

    for page in pages {
        md_content.push_str("\n");

        let page_name = md_path_to_name(page);
        md_content.push_str(&format!(
            "[{}]({}.html)",
            uppercase_first_letter(&page_name),
            &page_name
        ))
    }
    md_content.push_str("\n\n\n---\n");
    return md_content;
}

fn create_file(title: &str, content: &str, output_name: &str) {
    let file = File::open("contents/template-index.html").unwrap();
    let reader = BufReader::new(file);
    let mut contents = String::new();
    for l in reader.lines() {
        let line = l.unwrap();

        let output = if line.contains(TITLE_REPLACEMENT) {
            line.replace(TITLE_REPLACEMENT, title)
        } else if line.contains(CONTENT_REPLACEMENT) {
            line.replace(CONTENT_REPLACEMENT, &content)
        } else {
            line
        };

        contents.push_str(&output);
    }

    fs::write(format!("../{}.html", output_name), contents).unwrap();
}

fn generate_index(title: &str, header_md_content: &str, body_md_content: &str) {
    let header_content = markdown_to_html(&header_md_content, &get_comrak_options());
    let body_content = markdown_to_html(&body_md_content, &get_comrak_options());

    let mut content = String::new();
    content.push_str(&header_content);
    content.push_str(&body_content);

    create_file(&title, &content, "index");
}

fn generate_sub_page(header_md_content: &str, md_path: &PathBuf) {
    let name = md_path_to_name(&md_path);
    println!("Page: {:?}", name);

    let mut md_content = String::new();

    md_content.push_str(header_md_content);
    md_content.push_str(&fs::read_to_string(md_path).unwrap());

    let content = markdown_to_html(&md_content, &get_comrak_options());

    create_file(&name, &content, &name);
}

fn generate_sub_page_post(header_md_content: &str, post: &Post) {
    println!("{:?}", &post.name);

    let mut md_content = String::new();

    md_content.push_str(header_md_content);
    md_content.push_str(&format!("### {}\n", &post.title));
    md_content.push_str(&format!("{} :: {}\n", &post.date, &post.author));
    md_content.push_str(&post.content);

    let content = markdown_to_html(&md_content, &get_comrak_options());

    create_file(&post.title, &content, &post.name);
}

fn generate_md_post_list(list: &Vec<Post>) -> String {
    let mut content = String::new();
    content.push_str("#### Posts\n");

    for post in list {
        content.push_str(&format!(
            "[{}]({}.html) {}\n\n",
            &post.title, &post.name, &post.date
        ))
    }

    return content;
}

fn main() {
    println!("Starting");
    let page_title = "Gonçalo Palaio — Blog";
    let index_title = "Gonçalo's Blog";
    let md_posts = get_md_files(POST_PREFIX);
    let md_unlisted_posts = get_md_files(UNLISTED_POST_PREFIX);
    let md_pages = get_md_files(PAGE_PREFIX);

    let posts = read_posts(&md_posts);
    let unlisted_posts = read_posts(&md_unlisted_posts);

    let header_md_content = create_md_header(&index_title, &md_pages);

    let md_post_list = generate_md_post_list(&posts);

    for md in &md_pages {
        generate_sub_page(&header_md_content, &md);
    }

    for post in &posts {
        generate_sub_page_post(&header_md_content, &post);
    }
    
    for post in &unlisted_posts {
        generate_sub_page_post(&header_md_content, &post);
    }

    generate_index(&page_title, &header_md_content, &md_post_list);
}
