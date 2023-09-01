pub struct IP {
	ip: String,
}

const ERROR_STRING_INVALID_IP: &str = "Invalid IP address";
const ERROR_STRING_INVALID_PORT: &str = "Invalid port number";
const ERROR_STRING_INVALID_NO_PORT: &str = "No port number provided";
const ERROR_STRING_INVALID_IP_OCT1: &str = "Invalid IP Address (octet 1)";
const ERROR_STRING_INVALID_IP_OCT2: &str = "Invalid IP Address (octet 2)";
const ERROR_STRING_INVALID_IP_OCT3: &str = "Invalid IP Address (octet 3)";
const ERROR_STRING_INVALID_IP_OCT4: &str = "Invalid IP Address (octet 4)";

impl IP {
	const OCTET_MAX: u32 = 255;
	const PORT_MAX: u32 = 65535;
	
	pub fn new(ip: &str) -> Result<Self, &'static str> {
		match Self::validate(ip) {
			Ok(()) => {
				Ok(
					Self {
						ip: String::from(ip),
					}
				)
			},
			Err(e) => {
				Err(e)
			},
		}
	}
	
	pub fn get(&self) -> String {
		self.ip.clone()
	}
	
	fn validate(ip: &str) -> Result<(), &'static str> {
		let mut fields: Vec<_> = ip.split(".").collect();
		
		if fields.len() != 4 {
			return Err(ERROR_STRING_INVALID_IP);
		}
		
		let s: Vec<_> = fields[3].split(":").collect();
		
		if s.len() != 2 {
			return Err(ERROR_STRING_INVALID_NO_PORT);
		}
		
		fields[3] = s[0];
		fields.push(s[1]);
		
		if Self::validate_field(fields[0], Self::OCTET_MAX) == false {
			return Err(ERROR_STRING_INVALID_IP_OCT1);
		}
		if Self::validate_field(fields[1], Self::OCTET_MAX) == false {
			return Err(ERROR_STRING_INVALID_IP_OCT2);
		}
		if Self::validate_field(fields[2], Self::OCTET_MAX) == false {
			return Err(ERROR_STRING_INVALID_IP_OCT3);
		}
		if Self::validate_field(fields[3], Self::OCTET_MAX) == false {
			return Err(ERROR_STRING_INVALID_IP_OCT4);
		}
		if Self::validate_field(fields[4], Self::PORT_MAX) == false {
			return Err(ERROR_STRING_INVALID_PORT);
		}

		Ok(())
	}

	fn validate_field(value: &str, max: u32) -> bool {
		if let Ok(v) = value.parse::<u32>() {
			if v > max {
				return false;
			} else {
				return true;
			}
		} else {
			return false;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn ip_valid() {
		let test_ip = String::from("192.168.0.1:8094");
		
		if let Ok(ip) = IP::new(&test_ip) {
			assert_eq!(
				ip.get(),
				test_ip,
			);
		} else {
			assert!(false);
		}
	}

	#[test]
	fn ip_invalid_ip() {
		match IP::new("abc") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_IP);
			},
		}
	}
	
	#[test]
	fn ip_invalid_no_port() {
		match IP::new("192.168.0.1") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_NO_PORT);
			},
		}
	}

	#[test]
	fn ip_invalid_oct1() {
		match IP::new("256.168.0.1:8094") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_IP_OCT1);
			},
		}
	}

	#[test]
	fn ip_invalid_oct2() {
		match IP::new("192.256.0.1:8094") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_IP_OCT2);
			},
		}
	}
	
	#[test]
	fn ip_invalid_oct3() {
		match IP::new("192.168.256.1:8094") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_IP_OCT3);
			},
		}
	}
	
	#[test]
	fn ip_invalid_oct4() {
		match IP::new("192.168.0.256:8094") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_IP_OCT4);
			},
		}
	}
	
	#[test]
	fn ip_invalid_port() {
		match IP::new("192.168.0.1:66000") {
			Ok(_) => {
				assert!(false);
			},
			Err(e) => {
				assert_eq!(e, ERROR_STRING_INVALID_PORT);
			},
		}
	}
}