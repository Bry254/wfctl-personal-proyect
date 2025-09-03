use wayfire_rs::{ipc::WayfireSocket, models::View};
use serde_json::to_string_pretty;
use std::env;

async fn get_view_usable(socket : &mut WayfireSocket, args : &Vec<String>) -> Result<View, String> {
    for v in args {
        if v.contains("=") {
            let mut a : Vec<String> = v.split("=").map(String::from).collect();
            a[1] = a[1].replace("\'", "").replace("\"", "");
            let vistas =  socket.list_views().await.unwrap();
            match a[0].as_str() {
                "id" => {
                    let value: i64 = a[1].parse().unwrap();
                    for vista in vistas {
                        if vista.id == value {return Ok(vista)}
                    }
                },
                "name" => {
                    for vista in vistas {
                        if vista.title.to_lowercase().contains(&a[1].to_lowercase()) {return Ok(vista)}
                    }
                },
                "pid" => {
                    let value: i64 = a[1].parse().unwrap();
                    for vista in vistas {
                        if vista.pid == value {return Ok(vista);}
                    }
                },
                "program" => {
                    for vista in vistas {
                        if vista.app_id.contains(&a[1]) {return Ok(vista)}
                    }
                }
                _ => {}
            }
        }
    }
    return Err("Error".to_string());
}

#[tokio::main]
async fn main() {
    let mut args : Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() > 0 {
        let mut socket = WayfireSocket::connect().await.unwrap();
        match get_view_usable(&mut socket, &args).await {
         Ok(vista) => {
                     match args[0].as_str() {
            "getcursor" => {
                socket.get_cursor_position().await.unwrap();
            }
            "wshow" => {
                socket.expo_toggle().await.unwrap();
            }
            "get" => {
                println!("{}", to_string_pretty(&vista).unwrap())
            },
            "ls" => {
                let lista : Vec<View> = socket.list_views().await.unwrap();
                for i in lista {
                    println!("{}:{}:{}: {}: {}", i.id,i.pid ,i.app_id, i.title, i.type_field)
                }
            },
            "list" => {
                let lista : Vec<View> = socket.list_views().await.unwrap();
                println!("{}", to_string_pretty(&lista).unwrap());
            },
            "move" => {
                let x:i64 = args[1].parse().unwrap();
                let y:i64 = args[2].parse().unwrap();
                socket.configure_view(vista.id, x, y, vista.geometry.width, vista.geometry.height, None).await.unwrap();
            },
            "translate" => {
                let mut x:i64 = args[1].parse().unwrap();
                let mut y:i64 = args[2].parse().unwrap();
                x += vista.geometry.x.unwrap();
                y += vista.geometry.y.unwrap();
                socket.configure_view(vista.id, x, y, vista.geometry.width, vista.geometry.height, None).await.unwrap();
            },
            "resize" => {
                let w:i64 = args[1].parse().unwrap();
                let h:i64 = args[2].parse().unwrap();
                socket.configure_view(vista.id, vista.geometry.x.unwrap(), vista.geometry.y.unwrap(), w, h, None).await.unwrap();
            },
            "config" => {
                let x:i64 = args[1].parse().unwrap();
                let y:i64 = args[2].parse().unwrap();
                let w:i64 = args[3].parse().unwrap();
                let h:i64 = args[4].parse().unwrap();
                socket.configure_view(vista.id, x, y, w, h, None).await.unwrap();
            },
            "focus" => {
                println!("{} : {}", vista.app_id, vista.title);
                socket.set_focus(vista.id).await.unwrap();
            },
            "showapps" => {
                socket.scale_toggle().await.unwrap();
            },
            "showallapps" => {
                socket.scale_toggle_all().await.unwrap();
            }
            "set" => {
                match args[1].as_str() {
                    "fullscreen" => {
                        socket.set_view_fullscreen(vista.id, !vista.fullscreen).await.unwrap();
                        print!("Pantalla completa - ")
                    },
                    "hide" => {
                        socket.set_view_minimized(vista.id, true).await.unwrap();
                        print!("Minimizar - ")
                    },
                    "show" => {
                        socket.set_view_minimized(vista.id, false).await.unwrap();
                        print!("Maximizar - ")
                    },
                    "toggleshow" => {
                        socket.set_view_minimized(vista.id, !vista.minimized).await.unwrap();
                        print!("Mostrar/ocultar - ")
                    },
                    "close" => {
                        socket.close_view(vista.id).await.unwrap();
                        print!("Cerrar - ")
                    },
                    "back" => {
                        socket.send_view_to_back(vista.id, true).await.unwrap();
                        print!("Atras - ")
                    },
                    "sticky" => {
                        socket.set_view_sticky(vista.id, !vista.sticky).await.unwrap();
                        print!("Sticky -")
                    },
                    "top" => {
                        socket.set_view_always_on_top(vista.id, true).await.unwrap();
                        print!("on_top - ")
                    },
                    "untop" => {
                        socket.set_view_always_on_top(vista.id, false).await.unwrap();
                        print!("off_top - ")
                    }
                    "pin" => {
                        socket.set_view_sticky(vista.id, !vista.sticky).await.unwrap();
                        socket.set_view_always_on_top(vista.id, !vista.sticky).await.unwrap();
                        print!("pin - ")
                    },
                    "alpha" => {
                        let alpha = args[2].parse().unwrap();
                        socket.set_view_alpha(vista.id, alpha).await.unwrap();
                        print!("alpha - ")
                    }
                    _ => {
                    }
                }
                println!("{}:{}", vista.app_id,vista.title);
            },
            "find" => {
                match args[1].as_str() {
                    "name" => {
                        for i in socket.list_views().await.unwrap() {
                            if i.title.contains(args[2].as_str()) {
                                println!("{}:{}:{}", i.id,i.pid ,i.title);
                            }
                        }
                        
                    },
                    "pid" => {
                        let value : i64 = args[2].parse().unwrap();
                        for i in socket.list_views().await.unwrap() {
                            if i.pid == value {
                                println!("{}:{}:{}", i.id,i.pid, i.title);
                            }
                        }

                    },
                    "program" => {
                        for i in socket.list_views().await.unwrap() {
                            if i.app_id.contains(args[2].as_str()) {
                                println!("{}:{}:{}", i.id,i.pid, i.title);
                            }
                        }
                    }
                    _ => {

                    }
                }
            }
            _ => {}
        }
         },
         Err(error)  => {
            println!("Error al encontrar ventana")
         }
        }

    } else {
        println!("Help");
        println!("View = id|name|pid|program");
        println!("Usage: wfctl [OPTIONS] <view>");
        println!("  wfctl [getcursor|wshow|get|ls|list|move|resize|translate|config|focus|showapp|showallapps]");
        println!("  wfctl set [fullscreen|hide|show|toggleshow|close|back|sticky|top|untop|pin|alpha]");
        println!("  wfctl find [name|pid|progam]");
    }
}