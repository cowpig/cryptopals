
const BASE_64: [char; 64] = [ 
	'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S',
	'T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l',
	'm','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4',
	'5','6','7','8','9','+','/'
];

struct Bytes<'a> {
	bytes: &<'a>[u8]
}

fn hex_to_base64(hex: &str) -> &str {
	
}

fn hex_to_bytes(hex: &str) -> Bytes {

}

fn base64(n: & [u8]) -> &str {
	let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
	// allocate a string of size n.len() / 3 (rounding up)
	// for each triplet in n, convert bytes to a u64, 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
