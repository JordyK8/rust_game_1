use sdl2::rect::{Rect, Point};
use sdl2::{pixels::Color, render::Texture};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::collections::VecDeque;
use std::time::Duration;
use sdl2::image::{InitFlag, LoadTexture};
use specs::prelude::*;
use specs_derive::Component;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position(Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: i32,
    y: i32,
}
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
struct Sprite {
    /// The specific spritesheet to render from
    spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    current_frame: usize,
    up_frames: Vec<Sprite>,
    down_frames: Vec<Sprite>,
    left_frames: Vec<Sprite>,
    right_frames: Vec<Sprite>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Player {
    position: Point,
    sprite: Rect,
    velocity: Velocity,
    movement: VecDeque<char>,
    direction: Direction,
    current_frame: i32,
}

fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}
fn add_movement(player:&mut Player, pos: char) {
    let index = player.movement.contains(&pos);
    if !index {    
        player.movement.push_front(pos);
        player.movement.pop_back();
    }
    println!("Array {:?}", player.movement);
}

fn remove_movement(player:&mut Player, pos: char) {
    let index = player.movement.iter().position(|&r| r == pos).unwrap();
    player.movement.remove(index);
    player.movement.push_back('n');
    println!("Array {:?}", player.movement);
}

fn update_player(player: &mut Player) {    
    match player.movement[0] {
        'u' => {
            player.velocity.y = -PLAYER_MOVEMENT_SPEED; 
            player.velocity.x = 0;
            player.direction = Direction::Up;
        },
        'd' => {
            player.velocity.y = PLAYER_MOVEMENT_SPEED; 
            player.velocity.x = 0;
            player.direction = Direction::Down;
        },
        'l' => {
            player.velocity.x = -PLAYER_MOVEMENT_SPEED; 
            player.velocity.y = 0;
            player.direction = Direction::Left;
        },
        'r' => {
            player.velocity.x = PLAYER_MOVEMENT_SPEED; 
            player.velocity.y = 0;
            player.direction = Direction::Right;
        },
        _ => {
            player.velocity.x = 0;
            player.velocity.y = 0;
        }
        
    }
    if player.movement.contains(&'u') &&player.movement.contains(&'d') {
        player.velocity.y = 0;
    }
    if player.movement.contains(&'l') &&player.movement.contains(&'r') {
        player.velocity.x = 0;
    }
    // Only continue to animate if the player is moving
    if player.velocity.x > 0 && player.velocity.y > 0 {
        // Cheat: using the fact that all animations are 3 frames (NOT extensible)
        player.current_frame = (player.current_frame + 1) % 3;
    }
    player.position = player.position.offset(player.velocity.x, player.velocity.y);
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as  i32 * direction_spritesheet_row(player.direction),
        frame_width,
        frame_height,
    );

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;


    // Setting movements veqdeque
    let mut movement = VecDeque::new();
    let movements = vec!['n','n','n','n'];
    for item in movements {
        movement.push_front(item)
    }

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        velocity: Velocity { x: 0, y: 0 },
        movement,
        direction: Direction::Right,
        current_frame: 0,
    };


    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    add_movement(&mut player, 'l');
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    add_movement(&mut player, 'r');
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    add_movement(&mut player, 'd');
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    add_movement(&mut player, 'u');
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    remove_movement(&mut player, 'l');
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. }  => {
                    remove_movement(&mut player, 'r');
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. }  => {
                    remove_movement(&mut player, 'u');
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    remove_movement(&mut player, 'd');
                },
                _ => {
                }
            }
        }
        // Update
        i = (i + 1) % 255;
        update_player(&mut player);
        
        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;        

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}