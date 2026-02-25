slint::include_modules!();
use sysinfo::System;

fn main() -> Result<(), slint::PlatformError> {
    //stars system
    let mut sys = System::new_all();
    
    let used_ram = sys.used_memory() as f32;
    let total_ram = sys.total_memory() as f32;
    
    let used_swap = sys.used_swap() as f32;
    let total_swap = sys.used_swap() as f32;
    
    let system_name = System::name().unwrap_or_else(|| "Linux".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Linux".to_string());

    sys.free_swap();
    let swap_percentage = (used_swap as f32 / total_swap as f32);
    
    // Refresh
    sys.refresh_memory();
    let ram_percentage = (used_ram as f32 / total_ram as f32);
    
    
    // Create app instance    
    let ui = AppWindow::new()?;
    

    ui.set_ram_load(ram_percentage);
    ui.set_swap_load(swap_percentage);
    ui.set_system_name(system_name.into());
    ui.set_kernel_version(kernel_version.into());

    ui.run()
}

