import serial

with serial.Serial("/dev/rfcomm0", 9600, timeout=5) as ser:
    while True:
        line = ser.readline()  # read a '\n' terminated line
        print(line)
