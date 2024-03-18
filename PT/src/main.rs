extern crate raylib;
extern crate rodio;
extern crate threadpool;

use std::io::BufReader;
use std::thread;
use raylib::ffi::{LoadImage, WindowShouldClose};
use rodio::{Decoder, OutputStream, Sink};
use raylib::{ffi::PlaySound, prelude::*};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::process::Command;
use std::ffi::CString;

fn main() {

    pub struct Vec2F{
        pub x: f32,
        pub y: f32,
    }
    pub struct Vec3F{
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    pub struct Vec4F{
        pub w: f32,
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    pub struct Vec1F{
        pub a: f32,
    }

    let speed: f32 = 20.0;
    let sound_effect_vol = 1.25;
    // Get a single output stream handle to the default physical sound device
    let (_stream, handle) = OutputStream::try_default().unwrap();
    // Spawn a thread to play the geigercounter sound
    let geigercounterthread = thread::spawn({    
        let handlea = handle.clone();
        move || {
            let sinkd = Sink::try_new(&handlea).unwrap();
            let geigercounter = std::fs::File::open("assets/geigercounter.mp3").unwrap();
            sinkd.append(Decoder::new(BufReader::new(geigercounter)).unwrap());
            sinkd.set_volume(0.125);
            sinkd.sleep_until_end();
        }
    });

    let backthread = thread::spawn({
        let handleb = handle.clone();
        move || {
            let sinkc = Sink::try_new(&handleb).unwrap();
            let back = std::fs::File::open("assets/geigercounter.mp3").unwrap();
            sinkc.append(Decoder::new(BufReader::new(back)).unwrap());
            sinkc.set_volume(0.03125);
            sinkc.sleep_until_end();
        }
    });
    macro_rules! leverp {
        () => {
            let sinka = rodio::Sink::try_new(&handle).unwrap();
            let filea = std::fs::File::open("assets/lever.mp3").unwrap();
            sinka.append(rodio::Decoder::new(BufReader::new(filea)).unwrap());
            sinka.set_volume(sound_effect_vol);
            sinka.sleep_until_end();
        };
    }
    pub const idk: Color = Color {
        r: 30,
        g: 30,
        b: 30,
        a: 255,
    };
    pub const white: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    macro_rules! step {
        () => {
            let sinkb = rodio::Sink::try_new(&handle).unwrap();
            let fileb = std::fs::File::open("assets/steps.mp3").unwrap();
            sinkb.append(rodio::Decoder::new(BufReader::new(fileb)).unwrap());
            sinkb.set_volume(sound_effect_vol);
            sinkb.sleep_until_end();
        };
    }

    let (mut rl, thread) = raylib::init()
        .size(1000, 800)
        .title("name")
        .build();
    let mut player: Texture2D = rl.load_texture(&thread, "assets/player.png").unwrap();
    let mut r1: Texture2D = rl.load_texture(&thread, "assets/reactor1.png").unwrap();
    let mut r2: Texture2D = rl.load_texture(&thread, "assets/reactor2.png").unwrap();
    let mut train: Texture2D = rl.load_texture(&thread, "assets/train.png").unwrap();
    let mut reactor: Texture2D;
    let mut lever: Texture2D = rl.load_texture(&thread, "assets/lever.png").unwrap();
    let mut lever_turned: Texture2D = rl.load_texture(&thread, "assets/lever_turned.png").unwrap();
    let mut playerX = 0.0;
    let mut playerY = 0.0;
    let mut playerR: Rectangle = Rectangle::new(playerX, playerY, 80.0, 80.0);
    let mut rR: Rectangle = Rectangle::new(500.0, 400.0, 320.0, 320.0);
    let mut i: i8 = 0;
    let mut b: i8 = 0;
    let mut c: i8 = 0;
    let mut e: i8 = 0;
    let mut f: i8 = 0;
    let pyl: f32 = 301.0;
    let postxt1: Vector2 = Vector2::new(8.0, 770.0);
    let postxt2: Vector2 = Vector2::new(8.0, 30.0);
    while !rl.window_should_close() {
        println!("Spielerkoordinaten: X:{},Y:{}", playerX, playerY);
        rl.set_target_fps(74);
        if playerY >= 800.0{playerY = 799.0;}
        if playerX >= 1000.0{playerX = 999.0;}
        if playerY <= 0.0{playerY = 1.0;}
        if playerX <= 0.0{playerX = 1.0;}

        if rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON){
            rl.window_should_close();
            break;
        }
        if rl.is_key_down(KeyboardKey::KEY_W){
            step!();
            playerY = playerY - speed;
            playerR.y -= speed;
            println!("Schritt Vorwärts");
        }
        if rl.is_key_down(KeyboardKey::KEY_A){
            step!();
            playerX = playerX - speed;
            playerR.x -= speed;
            println!("Schritt Nach Links");

        }
        if rl.is_key_down(KeyboardKey::KEY_S){
            step!();
            playerY = playerY + speed;
            playerR.y += speed;
            println!("Schritt Rückwärts");

        }
        if rl.is_key_down(KeyboardKey::KEY_D){
            step!();
            playerX = playerX + speed;
            playerR.x += speed;
            println!("Schritt Nach Rechts");

        }
        if rl.is_key_down(KeyboardKey::KEY_F){
            if playerX == 101.0{
                if playerY == pyl{
                    b = 1;
                    println!("Betätigt");
                    leverp!();
                }
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_F){
            if playerX == 201.0{
                if playerY == pyl{
                    c = 1;
                    println!("Betätigt");
                    leverp!();
                }
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_F){
            if playerX == 301.0{
                if playerY == pyl{
                    e = 1;
                    println!("Betätigt");
                    leverp!();
                }
            }
        }
        if rl.is_key_down(KeyboardKey::KEY_F){
            if playerX == 401.0{
                if playerY == pyl{
                    f = 1;
                    println!("Betätigt");
                    leverp!();
                }
            }
        }
        let mut collision: bool;
        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        let mut fps: u32 = d.get_fps();
        let mut ct: String = format!("X:{}, Y:{}\nFPS:{}", playerX, playerY, fps);
        let mut coordstxt: &str = ct.as_str();
        d.clear_background(idk);
        let a = Rectangle::new(178.0, pyl+330.0, 403.0, 50.0);
        // 4 levers
        collision = playerR.check_collision_recs(&rR);

        d.draw_texture_pro(&train, a, a, Vector2::new(99.0, pyl-20.0), 0.0, white);
        if(b == 1){
            d.draw_texture_ex(&lever_turned,Vector2::new(101.0, pyl), 0.0, 5.0, white); 
        } else{
            d.draw_texture_ex(&lever,Vector2::new(101.0, pyl), 0.0, 5.0, white);
        }
        if(c == 1){
            d.draw_texture_ex(&lever_turned,Vector2::new(201.0, pyl), 0.0, 5.0, white); 
        } else{
            d.draw_texture_ex(&lever,Vector2::new(201.0, pyl), 0.0, 5.0, white);
        }
        if(e == 1){
            d.draw_texture_ex(&lever_turned,Vector2::new(301.0, pyl), 0.0, 5.0, white); 
        } else{
            d.draw_texture_ex(&lever,Vector2::new(301.0, pyl), 0.0, 5.0, white);
        }
        if(f == 1) {
            d.draw_texture_ex(&lever_turned,Vector2::new(401.0, pyl), 0.0, 5.0, white); 
        } else{
            d.draw_texture_ex(&lever,Vector2::new(401.0, pyl), 0.0, 5.0, white);
        }


        d.draw_texture_ex(&player, Vector2::new(playerX, playerY), 0.0, 5.0, white);
        if i <= 35 {
            d.draw_texture_ex(&r2,Vector2::new(500.0, 400.0), 0.0, 10.0, white);
        }
        if i >= 70 {
            i = 0;
            d.draw_texture_ex(&r1,Vector2::new(500.0, 400.0), 0.0, 10.0, white);
        }
        else{
            d.draw_texture_ex(&r1,Vector2::new(500.0, 400.0), 0.0, 10.0, white);
        }
        i = i + 1;
        d.draw_text("Ziel: Schalte den Reaktor ab",  8, 770, 20, Color::RED);
        d.draw_text(coordstxt, 8, 30, 20, Color::GREEN);
        let mut position: Vector2 = Vector2::new(playerX, playerY);
        let mut clear = Command::new("cls");
        //println!("{:#?}", position);
        if(collision){
            d.draw_rectangle(0, 0, 1000, 800, Color::RED);
        }
        
    }

    backthread.join().unwrap();
    geigercounterthread.join().unwrap();
}
