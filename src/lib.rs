pub const BASE_64: [char; 64] = [ 
	'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S',
	'T','U','V','W','X','Y','Z','a','b','c','d','e','f','g','h','i','j','k','l',
	'm','n','o','p','q','r','s','t','u','v','w','x','y','z','0','1','2','3','4',
	'5','6','7','8','9','+','/'
];

pub const HEX: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
];

pub fn hex_to_base64(hex: &str) -> String {
	let mut output = String::new();
	
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

	for bytes in hex.as_bytes().chunks(3) {
		if bytes.len() == 3 {
			// println!("\nconverting '{}{}{}'", bytes[0] as char, bytes[1] as char, bytes[2] as char);
			// println!("bytes are {:04b}.{:04b}.{:04b}", digit(bytes[0]), digit(bytes[1]), digit(bytes[2]));

		    let idx1 = (digit(bytes[0]) << 2) | (digit(bytes[1]) >> 2);
			// println!("idx1 is {:06b} ({})\n", idx1, idx1);
            output.push(BASE_64[idx1]);

			let idx2 = ((digit(bytes[1]) & 0b0011) << 4) | digit(bytes[2]);
			// println!("idx2 is {:06b} ({})\n", idx2, idx2);
            output.push(BASE_64[idx2]);
		} else if bytes.len() == 2 {
		    let idx1 = (digit(bytes[0]) << 2) | (digit(bytes[1]) >> 2);
			// println!("idx1 is {}", idx1);
            output.push(BASE_64[idx1]);

			let idx2 = digit(bytes[1] & 0xFF) << 4;
			// println!("idx2 is {}", idx2);
            output.push(BASE_64[idx2]);
			output.push('=');
		} else {
		    output.push(BASE_64[digit(bytes[0]) << 2]);
			output.push('=');
			output.push('=');
		}
		println!("output is {}", output);
	}
	
	return output;
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
	}
}
