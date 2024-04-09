use std::ffi::CString;

use contourwall_lib::{command_0_show_leds, command_1_fill_solid, command_2_update_all, new_contour_wall};

fn main() {
	// let mut portname = "COM6";
	// let ports = serialport::available_ports().expect("No ports found!");
	// for p in &ports {
	// 	println!("{}", p.port_name);
	// 	if p.port_name == "COM6" {
	// 		portname = "COM6";
	// 		println!("Found {}", portname);
	// 	}
	// }
	
	// let name = "Ellie";
	// println!("Hello, {}!", name);
	// let mut input = String::new();
	
	// loop {
	// 	match io::stdin().read_line(&mut input) {
	// 		Ok(n) => {
	// 			if input.trim() == "led" {
	// 				serial.write(b"led").expect("Write failed");
	// 			}
	// 			if input.trim() == "exit" {
		// 				break;
	// 			}
	// 			else{
	// 				serial.write(input.as_bytes()).expect("Write failed");
	// 			}
	// 			println!("{} bytes read\n", n);
	// 			input.clear();
	// 		}
	// 		Err(error) => println!("error: {error}"),
	// 	}
	// }

	// let mut serial = serialport::new("COM6", 115200).open().expect("serialport open failed");
	// serial.write(&[255, 0, 5, 25, 225, 0, 215, 24, 4, 164, 135, 90, 56, 244, 123, 40, 22, 77, 58, 98, 56, 12, 95, 83, 92, 124, 243, 20, 241, 124, 235, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 0, 255, 255, 0]).expect("Write failed");

		let serial = new_contour_wall(CString::new("COM6").expect("CString::new failed").as_ptr(), 115200).serial;
		// println!("{:?}", serial);
		
	    // let c_buffer_ptr: *const u8 = [255, 0, 5, 25, 225, 0, 215, 24, 4, 164, 135, 90, 56, 244, 123, 40, 22, 77, 58, 98, 56, 12, 95, 83, 92, 124, 243, 20, 241, 124, 235, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 0, 255, 255, 255, 0, 0, 255, 255, 0, 255, 0, 255, 255, 255, 255, 255, 10, 255].as_ptr();
		let c_buffer_ptr: *const u8 = [92, 201, 143, 18, 234, 55, 176, 120, 67, 13, 87, 209, 46, 151, 103, 227, 33, 189, 74, 221, 3, 116, 198, 29, 84, 237, 162, 40, 145, 70, 208, 12, 127, 76, 184, 21, 228, 63, 173, 37, 91, 205, 8, 253, 108, 48, 195, 26, 138, 80, 179, 9, 244, 58, 169, 44, 220, 6, 133, 97, 232, 51, 156, 112, 190, 36, 141, 85, 213, 16, 239, 68, 177, 24, 226, 59, 165, 42, 234, 0, 124, 81].as_ptr();
		
		println!("{:?}", c_buffer_ptr);
		
		command_2_update_all(serial, c_buffer_ptr);
		
		// Define the color
		let color_ptr: *const u8 = [255, 255, 0].as_ptr(); // Example color [R, G, B]
		
		command_1_fill_solid(serial, color_ptr);
		
		command_0_show_leds(serial);

		let res = unsafe { serial.read() };
		
		println!("{:?}", res);
}
