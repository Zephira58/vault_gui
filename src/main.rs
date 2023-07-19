#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{net::IpAddr, str::FromStr, time::Duration, thread};
use eframe::egui;
use vault_gui::*;
use egui_notify::{Anchor, Toasts, Toast};


use std::sync::mpsc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "vault_gui",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    toasts: Toasts,
    closable: bool,
    duration: f32,

    server_bool: bool,
    ip: String,
    port: i32,

    login_bool: bool,
    username: String,
    password: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            toasts: Toasts::default().with_anchor(Anchor::BottomRight),
            closable: true,
            duration: 3.5,

            server_bool: false,
            login_bool: false,

            ip: "127.0.0.1".to_owned(),
            port: 3306,

            username: "".to_owned(),
            password: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            let cb = |t: &mut Toast| {
                //Callback for the toast
                t.set_closable(self.closable)
                    .set_duration(Some(Duration::from_millis((1000. * self.duration) as u64)));
            };

            if self.server_bool == false {
                ui.heading("Vault GUI");
                ui.label("Please enter the ip and port of the sql server below");
                ui.horizontal(|ui| {
                    ui.label("IP: ");
                    ui.text_edit_singleline(&mut self.ip);
                });

                ui.horizontal(|ui| {
                    ui.label("Port: ");
                    ui.add(egui::DragValue::new(&mut self.port).speed(1.0).clamp_range(0..=65535));
                });

                if ui.button("Connect").clicked() {
                    let ip_verified = validate_ip_address(&self.ip);
                    if ip_verified == false {
                        cb(self.toasts.error("Invalid IP Address!"));
                        println!("Invalid IP Address!");
                        return
                    }

                    println!("IP: {}", self.ip);
                    println!("Port: {}", self.port);
                    println!("Testing connection to server...");
                    
                    let ip: IpAddr = IpAddr::from_str(&self.ip).unwrap();
                    let port = self.port;
    
                    // Create a channel to communicate the result of the ping test
                    let (tx, rx) = mpsc::channel();
                    
                    //TODO: Fix this async code and try to make it not freeze the gui
                    thread::spawn(move || {
                        let result = tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(is_server_alive(ip, port as u16, 5));
                        tx.send(result).unwrap();
                    });
                    
                    if let Ok(result) = rx.recv_timeout(Duration::from_secs(30)) {
                        if result {
                            cb(self.toasts.success("Connection Successful!"));
                            self.server_bool = true;
                            println!("Connection Successful!")
                        } else {
                            cb(self.toasts.error("Connection Failed!"));
                            println!("Connection Failed!")
                        }
                    } else {
                        cb(self.toasts.error("Connection Timed Out!"));
                        println!("Connection Timed Out!")
                    }
                }
            } 

            if self.login_bool == false && self.server_bool == true {
                ui.label("Please enter your username and password below");
                ui.horizontal(|ui| {
                    ui.label("Username: ");
                    ui.text_edit_singleline(&mut self.username);
                });
                ui.horizontal(|ui| {
                    ui.label("Password: ");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                });
                if ui.button("Login").clicked() {
                    println!("Username: {}", self.username);
                    println!("Password: {}", self.password);
                }
            }
            
        });
        self.toasts.show(ctx); // Requests to render toasts
    }
}

