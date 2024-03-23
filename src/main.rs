extern crate raylib;
extern crate rodio;
extern crate threadpool;
extern crate rand;

use std::io::BufReader;
use std::thread;
use raylib::ffi::{LoadImage, WindowShouldClose};
use rodio::{Decoder, OutputStream, Sink};
use raylib::{ffi::PlaySound, prelude::*};
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::ffi::CString;
use rand::prelude::*;
use std::time::Duration;

fn main() {
    let speed: f32 = 4.0;  // 20.0
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

    macro_rules! leverp {
        () => {
            let sinka = rodio::Sink::try_new(&handle).unwrap();
            let filea = std::fs::File::open("assets/lever.mp3").unwrap();
            sinka.append(rodio::Decoder::new(BufReader::new(filea)).unwrap()); 
            sinka.set_volume(sound_effect_vol); 
            sinka.sleep_until_end(); 
        }; 
    } 
    pub const lasc: Color = Color { r: 74, g: 74, b: 74, a: 255 };
    pub const idk: Color = Color { r: 30, g: 30, b: 30, a: 255, }; 
    pub const white: Color = Color { r: 255, g: 255, b: 255, a: 255, }; 
    
    
    macro_rules! step { () => { 
            let sinkb = rodio::Sink::try_new(&handle).unwrap(); 
            let fileb = std::fs::File::open("assets/steps.mp3").unwrap();
            sinkb.append(rodio::Decoder::new(BufReader::new(fileb)).unwrap()); 
            sinkb.set_volume(sound_effect_vol); 
            //sinkb.sleep_until_end();
        }; 
    }

    
    
    macro_rules! reactorboom { 
        () => { 
            let sinkc = rodio::Sink::try_new(&handle).unwrap(); 
            let filec = std::fs::File::open("assets/reactorboom.mp3").unwrap();
            sinkc.append(rodio::Decoder::new(BufReader::new(filec)).unwrap()); 
            sinkc.set_volume(sound_effect_vol * 2.0); 
            sinkc.sleep_until_end();
        }; 
    }


    macro_rules! alarm { 
        () => { 
            let sinkd = rodio::Sink::try_new(&handle).unwrap(); 
            let filed = std::fs::File::open("assets/alarm.mp3").unwrap();
            sinkd.append(rodio::Decoder::new(BufReader::new(filed)).unwrap()); 
            sinkd.set_volume(sound_effect_vol * 2.0); 
            sinkd.sleep_until_end();
        }; 
    }

    
    let (mut rl, thread) = raylib::init()
        .size(1000, 800)
        .title("name")
        .build();
    let mut greenslime: Texture2D = rl.load_texture(&thread, "assets/gs.png").unwrap();
    let mut player: Texture2D = rl.load_texture(&thread, "assets/player.png").unwrap();
    let mut r1: Texture2D = rl.load_texture(&thread, "assets/reactor1.png").unwrap();
    let mut r2: Texture2D = rl.load_texture(&thread, "assets/reactor2.png").unwrap();
    let mut train: Texture2D = rl.load_texture(&thread, "assets/train.png").unwrap();
    let mut reactor: Texture2D;
    let mut lever: Texture2D = rl.load_texture(&thread, "assets/lever.png").unwrap();
    let mut lever_turned: Texture2D = rl.load_texture(&thread, "assets/lever_turned.png").unwrap();
    let mut playerX = 0.0;
    let mut playerY = 0.0;
    let mut playerR: Rectangle = Rectangle::new(playerX, playerY, 15.0, 90.0);
    let mut rR: Rectangle = Rectangle::new(500.0, 400.0, 320.0, 320.0);
    let mut i: i8 = 0;
    let mut b: i8 = 0;
    let mut c: i8 = 0;
    let mut e: i8 = 0;
    let mut f: i8 = 0;
    let pyl: f32 = 301.0;
    let postxt1: Vector2 = Vector2::new(8.0, 770.0);
    let postxt2: Vector2 = Vector2::new(8.0, 30.0);
    let mut dt: f32;


    let mut rd: u32 = 0;
    let mut lever_d = rand::thread_rng().gen_range(0..4);

    let mut dY0 = rand::thread_rng().gen_range(0..1000);
    let mut dX0 = rand::thread_rng().gen_range(0..800);
    let mut dYf0 = dY0 as f32;
    let mut dXf0 = dX0 as f32;

    let mut deathR0: Rectangle = Rectangle::new(dXf0, dYf0, 10.0, 10.0);
    let mut dY1 = rand::thread_rng().gen_range(0..1000);
    let mut dX1 = rand::thread_rng().gen_range(0..800);
    let mut dYf1 = dY1 as f32;
    let mut dXf1 = dX1 as f32;

    let mut deathR1: Rectangle = Rectangle::new(dXf1, dYf1, 10.0, 10.0);

    let mut dY2 = rand::thread_rng().gen_range(0..1000);
    let mut dX2 = rand::thread_rng().gen_range(0..800);
    let mut dYf2 = dY2 as f32;
    let mut dXf2 = dX2 as f32;

    let mut deathR2: Rectangle = Rectangle::new(dXf2, dYf2, 10.0, 10.0);


    let mut dY4 = rand::thread_rng().gen_range(0..1000);
    let mut dX4 = rand::thread_rng().gen_range(0..800);
    let mut dYf4 = dY4 as f32;
    let mut dXf4 = dX4 as f32;

    let mut deathR4: Rectangle = Rectangle::new(dXf4, dYf4, 10.0, 10.0);


    let mut fps: u32;
    let mut ct: String;
    let mut coordstxt: &str;

    let mut a: Rectangle;

    let mut position: Vector2;

    let mut col0: bool;
    let mut col1: bool;
    let mut col2: bool;
    let mut col3: bool;
    let mut colProj: bool;
    let mut radCol: bool;
    let mut collision: bool;

    let mut dY3 = rand::thread_rng().gen_range(0..1000);
    let mut dX3 = rand::thread_rng().gen_range(0..800);
    let mut dYf3 = dY3 as f32;
    let mut dXf3 = dX3 as f32;

    let mut deathR3: Rectangle = Rectangle::new(dXf3, dYf3, 10.0, 10.0);
    let mut reactorShutDown: bool = false;

    let ProjectileT: Texture2D = rl.load_texture(&thread, "assets/projectile.png").unwrap();   
    let mut ProjectileX: f32 = 0.0;
    let mut HitBoxProjectile: Rectangle = Rectangle::new(ProjectileX, 310.0, 13.0, 13.0);

    let mut rbx = rand::thread_rng().gen_range(0..1000);
    let mut rby = rand::thread_rng().gen_range(0..800);

    let mut RadBox: Rectangle = Rectangle::new(rbx as f32, rbx as f32, 100.0, 100.0); // 10²*2


    
    let mut LaserX1: f32 = 95.0;
    let mut LaserBox1: Rectangle = Rectangle::new(LaserX1, 0.0, 5.0, 800.0);


    let mut LaserX2: f32 = 195.0;
    let mut LaserBox2: Rectangle = Rectangle::new(LaserX2, 0.0, 5.0, 800.0);



    let mut LaserX3: f32 = 295.0;
    let mut LaserBox3: Rectangle = Rectangle::new(LaserX3, 0.0, 5.0, 800.0);



    let mut LaserX4: f32 = 395.0;
    let mut LaserBox4: Rectangle = Rectangle::new(LaserX4, 0.0, 5.0, 800.0);


    
    let mut LBD = 0;
    let mut ColLaserBox1: bool = false;
    let mut ColLaserBox2: bool = false;
    let mut ColLaserBox3: bool = false;
    let mut ColLaserBox4: bool = false;
    let mut deathLB: bool = false;

    let mut loopthroughwsc = 0;
    while !rl.window_should_close() {
        dt = rl.get_frame_time() * rl.get_fps() as f32;

        rl.set_target_fps(30);
        ProjectileX += 10.0;
        HitBoxProjectile.x += 10.0;    


        if rl.is_key_down(KeyboardKey::KEY_SPACE){
            playerY = playerY - speed*2.0;
            playerR.y -= speed*2.0;
            
            thread::sleep(Duration::from_millis(250));

            playerY += speed;
            playerR.y += speed;
        }
        if rl.is_key_down(KeyboardKey::KEY_W){
            step!();
            playerY = playerY - (speed as f32 * dt);
            playerR.y -= (speed as f32 * dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_A){
            step!();
            playerX = playerX - (speed as f32 * dt);
            playerR.x -= (speed as f32 * dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_S){
            step!();
            playerY = playerY + (speed as f32 * dt);
            playerR.y += (speed as f32 * dt);
        }
        if rl.is_key_down(KeyboardKey::KEY_D){
            step!();
            playerX = playerX + (speed as f32 * dt);
            playerR.x += (speed as f32 * dt);
        }


        let mut d: RaylibDrawHandle<'_> = rl.begin_drawing(&thread);


        
        ColLaserBox1 = playerR.check_collision_recs(&LaserBox1);
        ColLaserBox2 = playerR.check_collision_recs(&LaserBox2);
        ColLaserBox3 = playerR.check_collision_recs(&LaserBox3);
        ColLaserBox4 = playerR.check_collision_recs(&LaserBox4);

        if playerY >= 800.0{playerY = 799.0;}
        if playerX >= 1000.0{playerX = 999.0;}
        if playerY <= 0.0{playerY = 1.0;}
        if playerX <= 0.0{playerX = 1.0;}    

        if loopthroughwsc >= 3000{
            alarm!();
            reactorboom!();
            rd += 1;
        }

        
        if d.is_key_down(KeyboardKey::KEY_F){
            if playerX >= 100.0 && playerX <= 112.0{
                if playerY >= 300.0 && playerY <= 312.0{
                    b = 1;
                    println!("Betätigt1");
                    leverp!();
                    if(lever_d == 0 || lever_d == 4){
                        println!("lol gut");
                        reactorShutDown = true;
                    } else{
                        rd += 1;     
                        alarm!();
                        reactorboom!();               
                    }               
                }
            } else if playerX >= 200.0 && playerX <= 212.0{ 
                if playerY >= 300.0 && playerY <= 312.0{
                    c = 1;
                    println!("Betätigt2");
                    leverp!();
                    if(lever_d == 1){
                        println!("lol gut");
                        reactorShutDown = true;
                    } else{
                        rd += 1;    
                        alarm!();   
                        reactorboom!();
                    }
                }
            } else if playerX >= 300.0 && playerX <= 312.0{
                if playerY >= 300.0 && playerY <= 312.0{
                    e = 1;
                    println!("Betätigt3");
                    leverp!();
                    if(lever_d == 2){
                        println!("lol gut");
                        reactorShutDown = true;
                    } else{
                        rd += 1;
                        alarm!();
                        reactorboom!();
                    }
                }
            } else if playerX >= 400.0 && playerX <= 412.0{
                if playerY >= 300.0 && playerY <= 312.0{
                    f = 1;
                    println!("Betätigt4");
                    leverp!();
                    if(lever_d == 3){
                        println!("lol gut");
                        reactorShutDown = true;
                    } else{                    
                        rd += 1;
                        alarm!();
                        reactorboom!();
                    }
                }
            }
        } 

        fps = d.get_fps();
        ct = format!("X:{}, Y:{}\nFPS:{}", playerX, playerY, fps);
        coordstxt = ct.as_str();

        d.clear_background(idk); // clear bg
        a = Rectangle::new(178.0, pyl+330.0, 403.0, 50.0);

        // 4 levers
        collision = playerR.check_collision_recs(&rR);
       
        //d.draw_texture_ex(&ProjectileT, Vector2::new(ProjectileX, 310.0), 0.0, 2.0, white); // 10x10 image

        d.draw_texture_ex(&greenslime, Vector2::new(dXf0, dYf0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf1, dYf1), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf2, dYf2), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf3, dYf3), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf4, dYf4), 0.0, 20.0, white);


        d.draw_texture_ex(&greenslime, Vector2::new(dXf0 + 3.0, dYf0 + 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf1 + 3.0, dYf1 + 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf2 + 3.0, dYf2 + 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf3 + 3.0, dYf3 + 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf4 + 3.0, dYf4 + 3.0), 0.0, 20.0, white);



        d.draw_texture_ex(&greenslime, Vector2::new(dXf0 - 3.0, dYf0 - 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf1 - 3.0, dYf1 - 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf2 - 3.0, dYf2 - 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf3 - 3.0, dYf3 - 3.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf4 - 3.0, dYf4 - 3.0), 0.0, 20.0, white);


        d.draw_texture_ex(&greenslime, Vector2::new(dXf0 + 6.0, dYf0 + 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf1 + 6.0, dYf1 + 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf2 + 6.0, dYf2 + 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf3 + 6.0, dYf3 + 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf4 + 6.0, dYf4 + 6.0), 0.0, 20.0, white);



        d.draw_texture_ex(&greenslime, Vector2::new(dXf0 - 6.0, dYf0 - 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf1 - 6.0, dYf1 - 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf2 - 6.0, dYf2 - 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf3 - 6.0, dYf3 - 6.0), 0.0, 20.0, white);
        d.draw_texture_ex(&greenslime, Vector2::new(dXf4 - 6.0, dYf4 - 6.0), 0.0, 20.0, white);


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

        d.draw_texture_ex(&player, Vector2::new(playerX, playerY), 0.0, 5.0, white); // PLAYER DRAWER
        if(!reactorShutDown){
            if i <= 15 {
                d.draw_texture_ex(&r2,Vector2::new(500.0, 400.0), 0.0, 10.0, white);
            }
            if i >= 41 {
                i = 0;
                d.draw_texture_ex(&r1,Vector2::new(500.0, 400.0), 0.0, 10.0, white);
            }
            else{
                d.draw_texture_ex(&r1,Vector2::new(500.0, 400.0), 0.0, 10.0, white);
            }
        }


        

        if LBD <= 70{
            LBD += 1;
            println!("Not Laser");
            d.draw_rectangle(LaserX1 as i32, 0, 10, 800, lasc);
            d.draw_rectangle(LaserX2 as i32, 0, 10, 800, lasc);
            d.draw_rectangle(LaserX3 as i32, 0, 10, 800, lasc);
            d.draw_rectangle(LaserX4 as i32, 0, 10, 800, lasc);
        } else if LBD >= 70 {
            d.draw_rectangle(LaserX1 as i32, 0, 10, 800, Color::RED);
            d.draw_rectangle(LaserX2 as i32, 0, 10, 800, Color::RED);
            d.draw_rectangle(LaserX3 as i32, 0, 10, 800, Color::RED);            
            d.draw_rectangle(LaserX4 as i32, 0, 10, 800, Color::RED);
            
            if ColLaserBox1 || ColLaserBox2 || ColLaserBox3 || ColLaserBox4{
                deathLB = true;
                println!("LASER")
            }
            LBD += 1;
        } if LBD >= 180{
            d.draw_rectangle(LaserX1 as i32, 0, 10, 800, lasc);
            d.draw_rectangle(LaserX2 as i32, 0, 10, 800, lasc);
            d.draw_rectangle(LaserX3 as i32, 0, 10, 800, lasc);
            d.draw_rectangle(LaserX4 as i32, 0, 10, 800, lasc);
            LBD = 0;
        }



        
        i = i + 1;
        d.draw_text("Ziel: Schalte den Reaktor ab",  8, 770, 20, Color::RED);
        d.draw_text(coordstxt, 8, 30, 20, Color::GREEN);
        position = Vector2::new(playerX, playerY);
        col0 = playerR.check_collision_recs(&deathR0);
        col1 = playerR.check_collision_recs(&deathR1);
        col2 = playerR.check_collision_recs(&deathR2);
        col3 = playerR.check_collision_recs(&deathR3);
        //colProj = playerR.check_collision_recs(&HitBoxProjectile);
        radCol = playerR.check_collision_recs(&RadBox);
        //println!("{:#?}", position);
        if(collision || col0 || col1 || col2 || col3 || /*colProj ||*/ radCol || deathLB){
            d.draw_rectangle(0,0,1000,800,Color::RED);
            d.draw_text("Du bist gestorben!", 250, 100, 40, Color::BLACK);
            if 1==1{
                d.window_should_close();
                break;
                rd += 1;
            }
        }

        if(ProjectileX >= 1000.0){ProjectileX = 0.0;}

        if(rd >= 1){
            d.draw_rectangle(0,0,1000,800,Color::RED);
            d.draw_text("Der Reaktor ist Explodiert!", 250, 100, 40, Color::BLACK);
            if 1==1{
                d.window_should_close();
                break;
                rd += 1;
            }
        }

        loopthroughwsc += 1;

                
        println!("{}", lever_d);
   }
   geigercounterthread.join().unwrap();
}
