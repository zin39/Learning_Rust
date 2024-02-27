//fn main() {
//    let a: i32 = 5;
//    let b: f64 = 6.7;
//    let d: &str = "my name is karan";
//    let e: bool = true;
//    println!("{} {} {} {}", a, b, d, e);
//}
//
//use std::io;
//fn main() {
//    println!("Input the sentene or word whatever u wish");
//    let mut input = String::new();
//    io::stdin()
//        .read_line(&mut input)
//        .expect("Faield to read line  ");
//    let char_count = input.trim().len();
//    let word_count = input.trim().split_whitespace().count();
//    let contain_rust = input.trim().to_lowercase().contains("rust");
//
//    println!("Input = {}", input);
//    println!("char count = {}", char_count);
//    println!("word count = {}", word_count);
//    println!("contain rust: {}", contain_rust);

struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

impl Book {
    fn new(title: &str, author: &str, publication_year: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            publication_year,
        }
    }

    fn display_details(&self) {
        println!("Title : {}", self.title);
        println!("Author : {}", self.author);
        println!("Publication year : {}", self.publication_year);
    }

    fn published_before_year(&self, year: u32) -> bool {
        self.publication_year > year
    }
}

fn main() {
    //create book instance
    let book1 = Book::new("Jame is Awesome", "jinny", 1996);
    let book2 = Book::new("Jame is Awesome1", "jinny1", 1997);
    let book3 = Book::new("Jame is Awesome2", "jinny2", 1998);
    println!("book1");
    book1.display_details();
    println!("book2");
    book2.display_details();
    println!("book3");
    book3.display_details();

    let year_to_check = 1990;
    println!(
        "Book 1 published before : {} : {}",
        year_to_check,
        book1.published_before_year(year_to_check)
    );

    println!(
        "Book 2 published before : {} : {}",
        year_to_check,
        book2.published_before_year(year_to_check)
    );
}
