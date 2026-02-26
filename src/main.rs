slint::include_modules!();
use std::thread;
use std::time::Duration;
use slint::Global;
use sysifo::Disks;
use sysinfo::{Disks, System};


fn main() -> Result<(), slint::PlatformError> {
    //stars system
    let mut sys = System::new_all(); 
    let mut disks = Disks::new_with_refreshed_list();

    let system_name = System::name().unwrap_or_else(|| "Linux".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Linux".to_string());
    
    // Refresh
    sys.refresh_cpu();
    sys.refresh_memory();

        
    // Create app instance    
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let timer = slint::Timer::default();

    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1000),move || {
        if let Some(ui) = ui_handle.upgrade(){
            //refresh
            sys.refresh_cpu();
            sys.refresh_memory();

            // Ram
            let used_ram = sys.used_memory() as f32;
            let total_ram = sys.total_memory() as f32;
            let ram_percentage = (used_ram as f32 / total_ram as f32);

            //CPu
            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let cpu_load = cpu_usage /100.0;

            //disks
            // for disk in &disks{
                // disk.mount_point();
                // disk.total_space();
                // disk.available_space()
            // }

            //uui
            ui.set_ram_load(ram_percentage);
            ui.set_cpu_load(cpu_load);
        }
    });

    ui.set_system_name(system_name.into());
    ui.set_kernel_version(kernel_version.into());

    ui.run()
}

