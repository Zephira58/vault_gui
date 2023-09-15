#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_notify::{Anchor, Toast, Toasts};
use std::{net::IpAddr, str::FromStr, thread, time::Duration};
use vault_gui::*;

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

    network_alive: bool,
    connect: bool,

    db_ip_valid: bool,
    ip: String,
    port: i32,

    login_bool: bool,
    username: String,
    password: String,

    tx: mpsc::Sender<bool>,
    rx: mpsc::Receiver<bool>,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            toasts: Toasts::default().with_anchor(Anchor::BottomRight),
            closable: true,
            duration: 3.5,

            network_alive: false,
            connect: false,

            db_ip_valid: false,
            login_bool: false,

            ip: "127.0.0.1".to_owned(),
            port: 3306,

            username: "".to_owned(),
            password: "".to_owned(),

            tx,
            rx,
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

            if self.network_alive == false {
                //Tests if the end user has internet access.
                let iptest = "142.250.70.142".to_owned();
                let ip: IpAddr = IpAddr::from_str(&iptest).unwrap();
                let port = 80;

                // Create a channel to communicate the result of the ping test
                let tx = self.tx.clone();

                thread::spawn(move || {
                    let result = tokio::runtime::Runtime::new()
                        .unwrap()
                        .block_on(is_server_alive(ip, port as u16, 30));
                    tx.send(dbg!(result)).expect("Failed to send result");
                });

                if let Ok(result) = self.rx.try_recv() {
                    if result {
                        cb(self.toasts.success("Network Connection Established!"));
                        println!("Network Connection Established");
                        self.network_alive = true
                    } else {
                        cb(self.toasts.error("Network Connection Failed!!"));
                        println!("Network Connection Failed!");
                    }
                }
            }

            if !self.db_ip_valid && self.network_alive {
                ui.heading("Vault GUI");
                ui.label("Please enter the ip and port of the sql server below");

                ui.horizontal(|ui| {
                    if ui.label("IP:").hovered() {
                        egui::show_tooltip(ui.ctx(), egui::Id::new("ip_tooltip"), |ui| {
                            ui.label("IPV4 Only");
                        });
                    }
                    ui.text_edit_singleline(&mut self.ip);
                });

                ui.horizontal(|ui| {
                    ui.label("Port:");
                    ui.add(
                        egui::DragValue::new(&mut self.port)
                            .speed(1.0)
                            .clamp_range(0..=65535),
                    );
                });

                if ui.button("Connect").clicked() {
                    self.connect = true;
                    cb(self.toasts.info("Testing connection..."));

                    let ip_verified = validate_ip_address(&self.ip);
                    if !ip_verified {
                        cb(self.toasts.error("Invalid IP Address!"));
                        println!("Invalid IP Address!");
                        return;
                    }

                    println!("IP: {}", self.ip);
                    println!("Port: {}", self.port);
                    println!("Testing connection to server...");

                    let ip: IpAddr = IpAddr::from_str(&self.ip).unwrap();
                    let port = self.port;

                    // Create a channel to communicate the result of the ping test
                    let tx = self.tx.clone();

                    thread::spawn(move || {
                        let result = tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(is_server_alive(ip, port as u16, 5));
                        tx.send(dbg!(result)).expect("Failed to send result");
                    });
                }
            }

            if !self.login_bool && self.db_ip_valid {
                ui.label("Please enter your username and password below");

                ui.horizontal(|ui| {
                    ui.label("Username:");
                    ui.text_edit_singleline(&mut self.username);
                });

                ui.horizontal(|ui| {
                    ui.label("Password:");
                    ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                });

                ui.horizontal(|ui| {
                    if ui.button("Login").clicked() {
                        println!("Username: {}", self.username);
                        println!("Password: {}", self.password);
                    }

                    if ui.button("Return").clicked() {
                        self.connect = false;
                        self.db_ip_valid = false;
                    }
                });
            }

            if self.network_alive && self.connect {
                if let Ok(result) = self.rx.try_recv() {
                    if result && !self.db_ip_valid {
                        cb(self.toasts.success("Connection Successful!"));
                        self.db_ip_valid = true;
                        println!("Connection Successful!")
                    } else if !self.db_ip_valid {
                        cb(self.toasts.error("Connection Failed!"));
                        println!("Connection Failed!")
                    }
                }
            }
        });
        self.toasts.show(ctx); // Requests to render toasts
    }
}
