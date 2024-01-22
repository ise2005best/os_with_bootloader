fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH"); // retrieves the UEFI_PATH from the rust env macro 
    println!("Path 1 {}", uefi_path);
    let bios_path = env!("BIOS_PATH");
    println!("Path 2 {}", bios_path);
    
    // choose whether to start the UEFI or BIOS image
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64"); // tells the compiler to execute the code on the qemu emulator
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive").arg(format!("format=raw,file={bios_path}"));
    }
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}