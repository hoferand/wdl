use crate::Value;

pub async fn print(val: Value) {
	println!("{}", val.to_string());
}
