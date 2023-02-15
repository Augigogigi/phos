use crate::prelude::*;

#[test]
fn test() {
	let frag1 = Pattern::new(unit(0).optional() +);

	println!("{:?}", frag1);
}