use alloc::vec::Vec;

use crate::println;

#[derive(Debug, Copy, Clone)]
pub struct PciDevice {
    bus: u8,
    device: u8,
    function: u8,
    vendor_id: u32,
    device_id: u32
}

impl PciDevice {
    pub fn read_bar(&self, bar_index: u8) -> u32 {
        let offset = 0x10 + (bar_index * 4);
        read_pci_register(self.bus, self.device, self.function, offset)
    }

    pub fn bus(&self) -> u8 {
        self.bus
    }

    pub fn device(&self) -> u8 {
        self.device
    }

    pub fn function(&self) -> u8 {
        self.function
    }

    pub fn vendor_id(&self) -> u32 {
        self.vendor_id
    }

    pub fn device_id(&self) -> u32 {
        self.device_id
    }
}


pub fn enumerate_pci_devices() -> Vec<PciDevice> {
    let mut devices = Vec::new();

    for bus in 0..=255 {
        for device in 0..=31 {
            for function in 0..=7 {
                let vendor_id = read_pci_register(bus, device, function, 0x00) & 0xFFFF;
                if vendor_id != 0xFFFF {
                    let device_id = (read_pci_register(bus, device, function, 0x00) >> 16) & 0xFFFF;
                    devices.push(PciDevice {
                        bus,
                        device,
                        function,
                        vendor_id,
                        device_id,
                    });
                }
            }
        }
    }
    
    devices
}


pub fn read_pci_register(bus: u8, slot: u8, function: u8, offset: u8) -> u32 {
    use x86_64::instructions::port::Port;

    let address = 0x80000000
        | ((bus as u32) << 16)
        | ((slot as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xfc);

    let mut config_address: Port<u32> = Port::new(0xCF8);
    let mut config_data: Port<u32> = Port::new(0xCFC);

    unsafe {
        config_address.write(address);
        config_data.read() as u32
    }
}

pub fn print_pci_devices(devices: &[PciDevice]) {
    for device in devices {
        println!(
            "Found device: bus = {}, BAR0 = {:#X}, device = {}, function = {}, vendor = {:#X}, device = {:#X} name: {}",
            device.bus, device.read_bar(0), device.device, device.function, device.vendor_id, device.device_id, get_device_name(device.vendor_id, device.device_id)
        );
    }
}

pub fn get_device_name(vendor_id: u32, device_id: u32) -> &'static str {
    match (vendor_id, device_id) {
        (0x8086, 0x1237) => "Intel 82441FX PMC [Natoma]",
        (0x8086, 0x7000) => "Intel 82371SB PIIX3 ISA [Natoma/Triton II]",
        (0x8086, 0x7010) => "Intel 82371SB PIIX3 IDE [Natoma/Triton II]",
        (0x8086, 0x7113) => "Intel 82371AB/EB/MB PIIX4 ACPI",
        (0x1234, 0x1111) => "QEMU PCI-PCI bridge",
        (0x8086, 0x100E) => "Intel 82540EM Gigabit Ethernet Controller",
        _ => "Unknown Device",
    }
}
