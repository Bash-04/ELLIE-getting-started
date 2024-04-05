import ctypes
from ctypes import c_char_p, c_uint32, c_uint8, c_void_p, POINTER

class ContourWallCore(ctypes.Structure):
    _fields_ = [("serial", c_void_p), ("baudrate", c_uint32)]

class ContourWall:
    def __init__(self, serial: str, baudrate=115_200):
        self.lib = ctypes.CDLL("../contourwall_lib/target/debug/contourwall_lib.dll")
        self.lib.new_contour_wall.argtypes = [c_char_p, c_uint32]
        self.lib.new_contour_wall.restype = ContourWallCore

        print("new_contour_wall" + str(self.lib.new_contour_wall))
        
        self.core = self.lib.new_contour_wall(serial.encode(), baudrate)
        print("core" + str(self.core))

        self.command_0_show_leds = self.lib.command_0_show_leds
# c_void_p is a pointer to the serial port (pointer)
        self.command_0_show_leds.argtypes = [POINTER(ContourWallCore)]
        self.command_0_show_leds.restype = c_uint8

        self.command_1_fill_solid = self.lib.command_1_fill_solid
# c_void_p is a pointer to the serial port (pointer) and c_uint8 is an byte array (pointer) with for 3 bytes (b, g, r) which is the color code to fill the entire panel
        self.command_1_fill_solid.argtypes = [POINTER(ContourWallCore), c_uint8]
        self.command_1_fill_solid.restype = c_uint8

        self.command_2_update_all = self.lib.command_2_update_all
# c_void_p is a pointer to the serial port (pointer) and c_uint8 is an byte array (pointer) with for each led 3 bytes (b, g, r)
        self.command_2_update_all.argtypes = [POINTER(ContourWallCore), c_uint8]    
        self.command_2_update_all.restype = c_uint8

        # Assigning the function to a different name to avoid conflict
        self.add_function = self.lib.add
        self.add_function.argtypes = [c_uint8, c_uint8]
        self.add_function.restype = c_uint8

    def show_leds(self):
        return self.command_0_show_leds(self.core) or 0
    
    def fill_solid(self, red, green, blue):
        return self.command_1_fill_solid(self.core, red, green, blue)
    
    def update_all(self, color_array):
        return self.command_2_update_all(self.core, color_array) or 0

    def add(self, a, b) -> int:
        return self.add_function(a, b)

    def __del__(self):
        self.lib
