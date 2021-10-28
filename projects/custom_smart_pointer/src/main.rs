use custom_smart_pointer::CustomSmartPointer;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    //c.drop(); //explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointers created.");
}
