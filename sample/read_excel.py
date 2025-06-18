from timeit import timeit
from sample_ext_lib import Book
from openpyxl import load_workbook

def read_excel_for_python():
    workbook = load_workbook("./sample/sample.xlsx")
    sheet = workbook["シート1"]
    return sheet["A1"].value

def read_excel_for_rust():
    book = Book("./sample/sample.xlsx")
    return book.get_value("シート1", "A1")

time_py = timeit(read_excel_for_python, number=100)
time_rust = timeit(read_excel_for_rust, number=100)

print(f"Python: {time_py:.6f} seconds")
print(f"Rust: {time_rust:.6f} seconds")


