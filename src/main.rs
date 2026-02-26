slint::include_modules!();
// use std::thread;
// use std::time::Duration;
// use slint::Global;
use sysinfo::{ Disks, Networks, System, };

fn main() -> Result<(), slint::PlatformError> {
    //Start System
    let mut sys = System::new_all(); 
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
    let mut cpu_values = vec![0.0; 100];
    let mut ram_values = vec![0.0; 100];

    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(1000),move || {
        if let Some(ui) = ui_handle.upgrade(){
            //Refresh Moduals
            sys.refresh_cpu();
            sys.refresh_memory();
            networks.refresh();

            //Load Ram Stats
            let used_ram = sys.used_memory() as f32;
            let total_ram = sys.total_memory() as f32;
            let ram_percentage = used_ram as f32 / total_ram as f32;

            ram_values.remove(0);
            ram_values.push(ram_percentage);

            let mut ram_path_string = String::new();
            let num_points = ram_values.len();

            for (i, &val) in ram_values.iter().enumerate() {
                let x = (i as f32 / (num_points - 1) as f32) * 100.0;
                let y = 100.0 - (val * 100.0);

                if i == 0 {
                    ram_path_string.push_str(&format!("M {} {} ", x, y));
                } else {
                    ram_path_string.push_str(&format!("L {} {} ", x, y));
                }
            }

            //Load Cpu Stats
            let cpu_usage = sys.global_cpu_info().cpu_usage();
            let cpu_load = cpu_usage / 100.0;

            cpu_values.remove(0);
            cpu_values.push(cpu_load);

            let mut cpu_path_string = String::new();
            let num_points = cpu_values.len();

            for (i, &val) in cpu_values.iter().enumerate() {
                let x = (i as f32 / (num_points - 1) as f32) * 100.0;
                let y = 100.0 - (val * 100.0);

                if i == 0 {
                    cpu_path_string.push_str(&format!("M {} {} ", x, y));
                } else {
                    cpu_path_string.push_str(&format!("L {} {} ", x, y));
                }
            }

            //Load Network Stats
            let mut total_rx = 0;
            let mut total_tx = 0;

            for (_interface_name, data) in &networks {
                total_rx += data.received();
                total_tx += data.transmitted();
            };

            ui.set_cpu_path_commands(cpu_path_string.into()); //Cpu Graph init
            ui.set_ram_path_commands(ram_path_string.into()); //Cpu Graph init

            let rx_mb = total_rx  as f32 / (1024.0 * 1024.0);
            let tx_mb = total_tx  as f32 / (1024.0 * 1024.0);

            //creates empy disk list
                let mut disks = Disks::new();
                //scans device for disks
                disks.refresh_list();

                for disk in &disks {
                    let _name = disk.name();
                    let _total_space = disk.total_space();
                }

                let disks = Disks::new_with_refreshed_list();

                let disk_models: Vec<DiskData> = disks.iter().map(|disk| {
                    let total = disk.total_space() as f32 / 1_073_741_824.0;
                    let available = disk.available_space() as f32 / 1_073_741_824.0;
                    let used = total - available;

                    let used_percentage = (used / total * 10.0).round() / 10.0;

                    DiskData{
                            name: disk.name().to_string_lossy().to_string().into(),
                            mount_point: disk.mount_point().to_string_lossy().to_string().into(),
                            total_space: (total * 100.0).round() / 100.0,
                            available_space: (available * 100.0).round() / 100.0,
                            space_used: (used * 100.0).round() / 100.0,
                            used_percent: used_percentage,
                    }
                }).collect();

                for disk in &disks {
                    let _name = disk.name();
                    let _total_space = disk.total_space();
                }

                let disk_model = slint::ModelRc::from(std::rc::Rc::new(slint::VecModel::from(disk_models)));
                ui.set_disks(disk_model);

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