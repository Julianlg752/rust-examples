
fn f1(s: &str) -> String {
    s.to_string()
}

fn f2(v1: &str,function_name: fn(&str) -> String) -> String {
    function_name(v1)
}

fn main() {
   let v1 = f1("hello");
   let v2 = f2("world", f1);
   println!("{}, {}", v1, v2);
}
