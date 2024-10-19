

fn main(){
	let mut c: u32 = 0x0;
	while c < 0x10FFFF {
		print!("{:?}", char::from_u32(c).unwrap());
		c += 1;
	}

}
