slint::include_modules!();
use std::thread;
use std::time::Duration;
use slint::Global;
use sysinfo::{ Disks, Networks, System};


fn main() -> Result<(), slint::PlatformError> {
    //Start System
    let mut sys = System::new_all(); 
    // let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();

    let system_name = System::name().unwrap_or_else(|| "Linux".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Linux".to_string());
    
    // Refresh
    sys.refresh_cpu();
    sys.refresh_memory();
    networks.refresh();
        

    // Create app instance    
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    let timer = slint::Timer::default();
    let mut cpu_values = vec![0.0f32; 60];

    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1000),move || {
        if let Some(ui) = ui_handle.upgrade(){
            //refresh
            sys.refresh_cpu();
            sys.refresh_memory();
            networks.refresh();

            // Ram
            let used_ram = sys.used_memory() as f32;
            let total_ram = sys.total_memory() as f32;
            let ram_percentage = (used_ram as f32 / total_ram as f32);

            //Cpu
            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let cpu_load = cpu_usage / 100.0;

            cpu_values.remove(0);
            cpu_values.push(cpu_load);

            let mut path_string = String::new();
            let num_points = cpu_values.len(); // This is a usize (e.g., 60)

            for (i, &val) in cpu_values.iter().enumerate() {
                // Fix: Use 'num_points' directly, don't call .len() on it!
                let x = (i as f32 / (num_points - 1) as f32) * 100.0;
                let y = 100.0 - (val * 100.0);

                if i == 0 {
                    path_string.push_str(&format!("M {} {} ", x, y));
                } else {
                    path_string.push_str(&format!("L {} {} ", x, y));
                }
            }

            ui.set_cpu_path_commands(path_string.into());

            // let model = std::rc::Rc::new(slint::VecModel::from(cpu_values.clone()));


            //disks
            // for disk in &disks{
                // disk.mount_point();
                // disk.total_space();
                // disk.available_space()
            // }

            // Network
            let mut total_rx = 0;
            let mut total_tx = 0;

            for (_interface_name, data) in &networks {
                total_rx += data.received();
                total_tx += data.transmitted();
            };

            let rx_mb = total_rx  as f32 / (1024.0 * 1024.0);
            let tx_mb = total_tx  as f32 / (1024.0 * 1024.0);

            //Load UI
            ui.set_net_rx_mb(rx_mb as f32);
            ui.set_net_tx_mb(tx_mb as f32);

            ui.set_ram_load(ram_percentage);
            ui.set_cpu_load(cpu_load);
        }
    });

    ui.set_system_name(system_name.into());
    ui.set_kernel_version(kernel_version.into());

    ui.run()
}

