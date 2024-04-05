use std::ffi::{c_char, CStr};

use serialport::SerialPort;

type SerialPortPtr = *mut Box<dyn SerialPort>;

#[repr(C)]
pub struct ContourWall {
    pub serial: SerialPortPtr,
    pub baudrate: u32,
}

#[no_mangle]
pub extern "C" fn new_contour_wall(com_port_ptr: *const c_char, baudrate: u32) -> ContourWall {
    let com_port = str_ptr_to_string(com_port_ptr);
    println!("Opening COM port: {}", com_port);
    let serial = serialport::new(&com_port, baudrate).open().expect(&format!("[CW CORE ERROR] Could not open COM port: {}", &com_port));
    ContourWall {
        serial: Box::into_raw(Box::new(serial)) as SerialPortPtr,
        baudrate: baudrate,
    }
}

#[no_mangle]
pub extern "C" fn command_0_show_leds(this: &mut ContourWall) {
    let serial = &mut this.serial;

    let buffer = [0];

    write_to_serial(serial.clone(), &buffer);

    // read_from_serial(serial.clone());
}

#[no_mangle]
/// 'color_ptr' is a pointer to an array of 3 u8 values representing the BRG color to fill the LEDs with.
/// For example: [[255, 0, 0]] is blue. [[0, 255, 0]] is red. [[0, 0, 255]] is green. [[255, 255, 255]] is white.
pub extern "C" fn command_1_fill_solid_rust(this: &mut ContourWall, color_ptr: *const u8) {
    let serial = &mut this.serial;

    let fill_color = unsafe { std::slice::from_raw_parts(color_ptr, 3) };

    // Create the buffer using a for loop
    let mut buffer: [u8; 76] = [0; 76]; // Initialize an array of 80 elements with zeros

    buffer[0] = 1; // Set the first element to 1 to indicate the command type

    // Fill the buffer with the desired color pattern
    for i in 1..76 {
        let color_index = i % 3; // Determine which color component to use based on position

        // Assign the appropriate color to the buffer element
        buffer[i] = fill_color[color_index];
    }

    write_to_serial(serial.clone(), &buffer);

    // read_from_serial(serial.clone());
}

#[no_mangle]
pub extern "C" fn command_1_fill_solid(this: &mut ContourWall, red: u8, green: u8, blue: u8) {
    let serial = &mut this.serial;

    let fill_color = [red, green, blue];

    println!("Fill color: {:?}", fill_color);

    // Create the buffer using a for loop
    let mut buffer: Vec<u8> = vec![1]; 

    // Fill the buffer with the desired color pattern
    for _i in 1..26 {
        // Assign the appropriate color to the buffer element
        buffer.push(fill_color[0]);
        buffer.push(fill_color[1]);
        buffer.push(fill_color[2]);
        // print!("{}, ", _i)
    }
    // println!("one: {}; two: {}; three: {}", fill_color[0], fill_color[1], fill_color[2]);
    
    // println!("Buffer length: {:?}", buffer.len());

    // println!("Buffer: {:?}", buffer);

    write_to_serial(serial.clone(), &buffer);

    // read_from_serial(serial.clone());
}

#[no_mangle]
/// 'c_buffer_ptr' is a pointer to an array of 75 u8 values representing the BRG color to fill the LEDs with.
/// This array is 75 elements long because it expects 3 color codes for each led, while there are 25 LEDs.
pub extern "C" fn command_2_update_all(this: &mut ContourWall, c_buffer_ptr: *const u8) {
    let serial = &mut this.serial;


    let buffer = unsafe { [2].iter().cloned().chain(std::slice::from_raw_parts(c_buffer_ptr, 75).iter().cloned()).collect::<Vec<u8>>()};

    println!("{:?}", buffer);

    write_to_serial(serial.clone(), &buffer);

    // read_from_serial(serial.clone());
}

fn str_ptr_to_string(ptr: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(ptr) };
    c_str.to_string_lossy().into_owned()
}

fn write_to_serial(serial: SerialPortPtr, bytes: &[u8]) {
    println!("Writing to serial port");
    unsafe { (*serial).write(bytes).expect("Could not write to serial port"); }
    println!("Wrote to serial port");
}

// fn read_from_serial(serial: SerialPortPtr) -> Vec<u8> {
//     let mut buffer: Vec<u8> = vec![0; 75];
//     unsafe { (*serial).read(buffer.as_mut_slice()).expect("Could not read from serial port"); }
//     println!("Read from serial port: {:?}", buffer);
//     buffer
// }

#[no_mangle]    // means the function name isn't changed by the compiler
pub extern "C" fn add(left: usize, right: usize) -> usize { // '-> usize' is the return type
    left + right  // no semicolon means this is the return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
