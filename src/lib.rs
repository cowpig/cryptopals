pub const BASE_64: [char; 64] = [ 
	'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S',
	'T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l',
	'm','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4',
	'5','6','7','8','9','+','/'
];

pub const HEX: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
];

fn add_triplet<'a>(output: &'a mut String, byte0: u8, byte1: u8, byte2: u8) -> &'a mut String {
	let digit = |byte| {
		// println!("converting {:08b} ({})", byte, byte);
		let dig = match byte {
			48...57 => (byte - 48) as usize, // 0 thru 9
			97...102 => (byte - 87) as usize, // a thru f
			65...70 => (byte - 55) as usize, // A thru F
			_ => panic!("invalid hex str")
		};
		// println!("converts to {:04b} ({})", dig, dig);
		dig
	};

	// println!("\nconverting '{}{}{}'", byte0 as char, byte1 as char, byte2 as char);
	// println!("bytes are {:04b}.{:04b}.{:04b}", digit(byte0), digit(byte1), digit(byte2));

	let idx1 = (digit(byte0) << 2) | (digit(byte1) >> 2);
	// println!("idx1 is {:06b} ({})\n", idx1, idx1);
	output.push(BASE_64[idx1]);

	let idx2 = ((digit(byte1) & 0b0011) << 4) | digit(byte2);
	// println!("idx2 is {:06b} ({})\n", idx2, idx2);
	output.push(BASE_64[idx2]);

	output
}

pub fn hex_to_base64(hex: &str) -> String {
	let mut output = &mut String::new();
	
	for bytes in hex.as_bytes().chunks(6) {

		if bytes.len() == 6 {
			add_triplet(output, bytes[0], bytes[1], bytes[2]);
			add_triplet(output, bytes[3], bytes[4], bytes[5]);
		} else if bytes.len() == 4 {
			add_triplet(output, bytes[0], bytes[1], bytes[2]);
			add_triplet(output, bytes[3], 48, 48);
			output.push('=');
		} else if bytes.len() == 2 {
			add_triplet(output, bytes[0], bytes[1], 48);
			output.push('=');
			output.push('=');
		} else {
			panic!("Got an odd numbered hex");
		}
		println!("output is {}", output);
	}
	
	return output.clone();
}

#[cfg(test)]
mod tests {
	use hex_to_base64;

	#[test]
	fn it_works() {
		let input1 = "000000";
		let output1 = "AAAA";
		assert!(hex_to_base64(input1) == output1);

		let input2 = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
		let output2 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
		assert!(hex_to_base64(input2) == output2);

		let input3 = "12342412";
		let output3 = "EjQkEg==";
		assert!(hex_to_base64(input3) == output3);
	}
}
