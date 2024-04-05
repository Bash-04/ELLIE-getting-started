import ctypes
from contourwall import ContourWall, ContourWallCore

cw = ContourWall("COM6", 115_200)

print('res:', cw.add(1, 2))

cw.fill_solid(0, 0, 255)

cw.show_leds()






# # Path: demo.py
# # Load the shared object library
# cwlc = ctypes.CDLL("../contourwall_lib/target/debug/contourwall_lib.dll")

# # Define the function signature
# contourwall_lib.new_contour_wall.argtypes = [ctypes.c_char_p, ctypes.c_uint32]
# contourwall_lib.new_contour_wall.restype = ctypes.c_void_p  # Assuming ContourWall is a pointer type

# # Define a wrapper function to call new_contour_wall
# def new_contour_wall(com_port_ptr, baudrate):
#     return contourwall_lib.new_contour_wall(com_port_ptr, baudrate)

# com_port = b"COM6" 
# baudrate = 115200
# contour_wall_ptr = new_contour_wall(com_port, baudrate)

# res = cwl.add(1, 2)

# print("res")

## check the basics to learn rust + python: https://medium.com/@afomalhaut/how-to-compile-dll-in-rust-and-import-it-in-python-useful-hints-466be4e7475b
