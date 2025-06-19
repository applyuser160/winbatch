#[cfg(test)]
mod tests {
    use super::super::book::Book;
    use umya_spreadsheet::{Spreadsheet, Worksheet, writer};
    use std::fs;

    fn create_test_excel(path: &str) {
        let mut book = Spreadsheet::default();
        let mut sheet = Worksheet::default();
        sheet.get_cell_mut("A1").set_value("Hello");
        sheet.get_cell_mut("B2").set_value("World");
        sheet.set_name("Sheet1");
        book.add_sheet(sheet);

        let mut sheet2 = Worksheet::default();
        sheet2.get_cell_mut("A1").set_value("Another");
        sheet2.set_name("Sheet2");
        book.add_sheet(sheet2);

        writer::xlsx::write(&book, path).unwrap();
    }

    #[test]
    fn test_new_and_sheetnames() {
        let path = "test_book_sheetnames.xlsx";
        create_test_excel(path);

        let book = Book::new(path.to_string());
        let sheetnames = book.sheetnames();
        assert_eq!(sheetnames, vec!["Sheet1".to_string(), "Sheet2".to_string()]);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_get_value() {
        let path = "test_book_get_value.xlsx";
        create_test_excel(path);

        let book = Book::new(path.to_string());
        assert_eq!(book.get_value("Sheet1".to_string(), "A1".to_string()), "Hello");
        assert_eq!(book.get_value("Sheet1".to_string(), "B2".to_string()), "World");
        assert_eq!(book.get_value("Sheet2".to_string(), "A1".to_string()), "Another");
        assert_eq!(book.get_value("NoSheet".to_string(), "A1".to_string()), "Sheet not found");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_repr() {
        let path = "test_book_repr.xlsx";
        create_test_excel(path);

        let book = Book::new(path.to_string());
        assert_eq!(book.__repr__(), format!("<Book path='{}'>", path));

        let _ = fs::remove_file(path);
    }
}