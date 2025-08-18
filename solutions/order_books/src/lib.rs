pub use library::writers::Writer;

pub mod library {
    pub mod writers {
        use super::books::Book;
        #[derive(Debug, Eq, PartialEq)]
        pub struct Writer {
            pub first_name: String,
            pub last_name: String,
            pub books: Vec<Book>,
        }
    }
    pub mod books {
        #[derive(Debug, Eq, PartialEq)]
        pub struct Book {
            pub title: String,
            pub year: u32,
        }
    }
}
pub fn order_books(writer: &mut Writer) {
    writer.books.sort_by(|a, b| a.title.cmp(&b.title));
}
