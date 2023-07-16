use quote::quote;
fn main() {
    let source = r#"
        fn main(){
            let s = String::from("hello");
            print(s);
        }

        fn print(s: String){
            println!("{}", s);
        }
    "#;

    let syntax = syn::parse_file(source).unwrap();
    let expanded = quote! {
        #syntax
    };
    println!("{}", expanded);
}

#[test]
fn test_single_move() {
    let source = r#"
        fn main(){
            let s = String::from("hello");
            print(s);
        }

        fn print(s: String){
            println!("{}", s);
        }
    "#;

    let result= r#"
        fn main(){
            let s = String::from("hello");
            print(s);
        }

        fn print(s: String){
            println!("{}", s);
        }
    "#;

    compare(source, result)
}

#[test]
fn test_double_move() {
    let source = r#"
        fn main(){
            let s = String::from("hello");
            print(s);
            print(s);
        }

        fn print(s: String){
            println!("{}", s);
        }
    "#;

    let result= r#"
        fn main(){
            let s = String::from("hello");
            print(&s);
            print(&s);
        }

        fn print(s: &String){
            println!("{}", *s);
        }
    "#;

    compare(source, result)
}

fn compare(source: &str, result: &str) {

    let syntax_source = syn::parse_file(source).unwrap();
    let expanded_source = quote! {
        #syntax_source
    };

    //We also parse the result to make sure it is valid syntax and compare the two regardless of formating
    let syntax_result = syn::parse_file(result).unwrap();
    let expanded_result = quote! {
        #syntax_result
    };

    assert_eq!(expanded_source.to_string(), expanded_result.to_string())
}