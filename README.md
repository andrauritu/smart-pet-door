# Smart Pet Door

## Description
The Smart Pet Door provides pets with the freedom to enter their home while ensuring security against unregistered animals. It utilizes a combination of BLE beacon technology and an infrared motion sensor to detect and authenticate a pet's approach and to automatically manage access.

## Software
To recreate this project, you should connect the hardware components as shown in the schematics, and follow the steps:
## 1. Clone then to the project directory
```
git clone https://github.com/UPB-FILS-MA/project-andrauritu.git
cd path/to/project/smart_pet_door/attempt4micro/embassy 
```
## 2. Build the project
```
cargo build --target thumbv6m-none-eabi
```
## 3. Mount the pico 
While connecting the Raspberyy Pi Pico W to the computer through a USB cable,  hold the BOOTSEL button down. 

## 4. Flash the firmware
Locate the binary in the target/thumbv6m-none-eabi/release/ folder and run:
```
elf2uf2-rs -d /path/to/your/binary
```

Note that there is no -s in the command, because we will connect the CP2102 USB to UART Bridge for serial communication.

## 5. Connect the USB to UART Bridge (Optional but recommended)
In order to check if everything works as intended, there are several debug messages through the code sent through UART to the CP2102 module. If you choose not to use it, make sure to connect the HM-10 module to a 3.3V output. 
After the firmware has been flashed onto the pico, connect the bridge to your computer through USB and monitor the serial output through a terminal. If you choose to use PuTTy, ensure that the baud rate is set to 9600 and that you have used the appropriate port.

### Possible issue
In case the output does not look ok in the PuTTy terminal, it might be because of a wrong baud rate for the HM-10. My HM-10 has the version 604, but in case yours comes updated to version 700 or newer, its default baud rate will be 115000. You will need to either change the code (config0 and config1) and set the baudrate variable to 115000, or change the HM-10's baudrate to 9600 by sending it the command ``` AT+BAUD0 ```. 
To check the current version of your module, send the command ``` AT+VERS?" ```.

## Configuring the NanoBeacon (Optional)
Although it does not affect the succesful running of the code, if you want to actually put this project to real-life use, you will need a beacon attached to your pet's collar. They are usually pricy, so the best alternative I could find in Romania was the IN100 NanoBeacon, which can be programmed through the InPlay application. Once you are satisfied with the configuration, you can burn it in and then use a CR1225 coin battery to power it while moving it freely, disconnected from the computer or breadboard. 

## Hardware

| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico WH](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | Acts as the central controller | [38.99 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/12395-raspberry-pi-pico-wh.html?search_query=pico+wh&results=32) |
| [HM-10 BLE Module](https://people.ece.cornell.edu/land/courses/ece4760/PIC32/uart/HM10/DSD%20TECH%20HM-10%20datasheet.pdf) | For scanning the NanoBeacon | [35.47 RON](https://cleste.ro/modul-bluetooth-4-0-ble.html) |
| [IN100 NanoBeacon](https://cdn.sparkfun.com/assets/3/d/5/5/1/IN100-Datasheet.pdf) | Attached to the pet’s collar | [44 RON](https://www.robofun.ro/wireless/breakout-sparkfun-nanobeacon-in100.html) |
| [Servomotor](https://datasheetspdf.com/pdf-down/S/G/9/SG90-TowerPro.pdf) | Operates the door lock | [12.72 RON](https://cleste.ro/motor-servo-sg90-9g.html) |
| [PIR Sensor HC-SR501](https://www.mpja.com/download/31227sc.pdf) | Detects motion at the door | [5.25 RON](https://www.robofun.ro/pir/hc-sr501-pir-motion-sensor-module-green.html) |


## Links

1. https://www.martyncurrey.com/hm-10-bluetooth-4ble-modules/
2. https://forum.arduino.cc/t/how-to-make-an-ibeacon-detector-with-arduino/278665/68
3. https://engineering.fresnostate.edu/research/bulldogmote/documents/11.%20HM10%20BLE_FTDI.pdf
4. https://magpi.raspberrypi.com/articles/raspberry-pi-cat-flap
