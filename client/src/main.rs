use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

struct Buttons {
    host_join_visible: bool,
    computer_visible: bool,
    table: bool,
    winner: bool,
    loser: bool,
    refresh: bool
}

impl Buttons {
    fn new() -> Self {
        Buttons {
            host_join_visible: true,
            computer_visible: false,
            table: false,
            winner: false,
            loser: false,
            refresh: false,
        }
    }
}

fn empty_hex(canvas: &mut WindowCanvas, centerx: i32, centery: i32, radius: i32) {
    let arr = [
        Point::new(centerx, centery - radius),
        Point::new(centerx + radius, centery - radius / 2),
        Point::new(centerx + radius, centery + radius / 2),
        Point::new(centerx, centery + radius),
        Point::new(centerx - radius, centery + radius / 2),
        Point::new(centerx - radius, centery - radius / 2),
    ];
    for i in 0..6 {
        canvas
            .draw_line(arr[i], arr[(i + 1) % 6])
            .ok()
            .unwrap_or_default();
    }
}

fn block_hex(canvas: &mut WindowCanvas, centerx: i32, centery: i32, radius: i32) {
    let arr = [
        Point::new(centerx, centery - radius),
        Point::new(centerx + radius, centery - radius / 2),
        Point::new(centerx + radius, centery + radius / 2),
        Point::new(centerx, centery + radius),
        Point::new(centerx - radius, centery + radius / 2),
        Point::new(centerx - radius, centery - radius / 2),
    ];
    for i in 0..6 {
        canvas
            .draw_line(arr[i], arr[(i + 1) % 6])
            .ok()
            .unwrap_or_default();
    }
    let bloc = Rect::new(centerx - 20, centery - 20, 40, 40);
    canvas.fill_rect(bloc).unwrap();
}

fn mouse_hex(canvas: &mut WindowCanvas, centerx: i32, centery: i32, radius: i32) {
    let arr = [
        Point::new(centerx, centery - radius),
        Point::new(centerx + radius, centery - radius / 2),
        Point::new(centerx + radius, centery + radius / 2),
        Point::new(centerx, centery + radius),
        Point::new(centerx - radius, centery + radius / 2),
        Point::new(centerx - radius, centery - radius / 2),
    ];
    for i in 0..6 {
        canvas
            .draw_line(arr[i], arr[(i + 1) % 6])
            .ok()
            .unwrap_or_default();
    }
    canvas.set_draw_color(Color::RGB(178, 190, 181));
    let mouse = Rect::new(centerx - 10, centery - 10, 20, 20);
    canvas.fill_rect(mouse).unwrap();
}

fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    buttons: &Buttons,
    board: [u8; 169],
) -> Result<(), String> {
    // Clear the canvas
    let color = Color::RGB(50, 205, 50);
    canvas.set_draw_color(color);
    canvas.clear();

    // Render buttons if visible
    if buttons.host_join_visible {
        // Render "Host" button
        let host_button_text: String = "Host".to_string();
        let host_surface = font
            .render(&host_button_text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let host_texture = texture_creator
            .create_texture_from_surface(&host_surface)
            .map_err(|e| e.to_string())?;
        let host_button_target = Rect::new(400, 375, 200, 50);
        canvas.set_draw_color(Color::RGB(0, 128, 0));
        canvas.fill_rect(host_button_target)?;
        canvas.copy(&host_texture, None, Some(host_button_target))?;

        // Render "Join" button
        let join_button_text: String = "Join".to_string();
        let join_surface = font
            .render(&join_button_text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let join_texture = texture_creator
            .create_texture_from_surface(&join_surface)
            .map_err(|e| e.to_string())?;
        let join_button_target = Rect::new(400, 455, 200, 50);
        canvas.set_draw_color(Color::RGB(0, 128, 0));
        canvas.fill_rect(join_button_target)?;
        canvas.copy(&join_texture, None, Some(join_button_target))?;
    }


    // Render "Computer" button if visible
    if buttons.computer_visible {
        let computer_button_text: String = "Start".to_string();
        let computer_surface = font
            .render(&computer_button_text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let computer_texture = texture_creator
            .create_texture_from_surface(&computer_surface)
            .map_err(|e| e.to_string())?;
        let computer_button_target = Rect::new(300, 50, 200, 50); // Adjust position as needed
        canvas.set_draw_color(Color::RGB(0, 128, 0));
        canvas.fill_rect(computer_button_target)?;
        canvas.copy(&computer_texture, None, Some(computer_button_target))?;
    }

    if buttons.refresh {
        let refresh_text: String = "Refresh".to_string();
        let refresh_surface = font
            .render(&refresh_text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let refresh_texture = texture_creator
            .create_texture_from_surface(&refresh_surface)
            .map_err(|e| e.to_string())?;
        let refresh_target = Rect::new(300, 50, 200, 50); // Adjust position as needed
        canvas.set_draw_color(Color::RGB(0, 128, 0));
        canvas.fill_rect(refresh_target)?;
        canvas.copy(&refresh_texture, None, Some(refresh_target))?;
    }

    if buttons.winner {
        let computer_button_text: String = "Winner".to_string();
        let computer_surface = font
            .render(&computer_button_text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let computer_texture = texture_creator
            .create_texture_from_surface(&computer_surface)
            .map_err(|e| e.to_string())?;
        let computer_button_target = Rect::new(300, 50, 200, 50); // Adjust position as needed
        canvas.set_draw_color(Color::RGB(0, 128, 0));
        canvas.fill_rect(computer_button_target)?;
        canvas.copy(&computer_texture, None, Some(computer_button_target))?;
    }
    if buttons.loser {
        let computer_button_text: String = "Loser".to_string();
        let computer_surface = font
            .render(&computer_button_text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let computer_texture = texture_creator
            .create_texture_from_surface(&computer_surface)
            .map_err(|e| e.to_string())?;
        let computer_button_target = Rect::new(300, 50, 200, 50); // Adjust position as needed
        canvas.set_draw_color(Color::RGB(0, 128, 0));
        canvas.fill_rect(computer_button_target)?;
        canvas.copy(&computer_texture, None, Some(computer_button_target))?;
    }
    // Render big brown button if visible
    if buttons.table {
        let table = Rect::new(130, 120, 825, 600);
        canvas.set_draw_color(Color::RGB(139, 69, 19)); // Brown color
        canvas.fill_rect(table)?;
        let mut j: i32 = 0;
        for i in 0..169 {
            if i % 13 == 0 && i > 0 {
                j += 1;
            }
            if j % 2 == 0 {
                if board[i as usize] == 0 || board[i as usize] == 1 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    empty_hex(canvas, 160 + (i % 13) * 61, 150 + j * 45, 30);
                } else if board[i as usize] == 2 {
                    mouse_hex(canvas, 160 + (i % 13) * 61, 150 + j * 45, 30);
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    block_hex(canvas, 160 + (i % 13) * 61, 150 + j * 45, 30);
                }
            } else if  board[i as usize] == 0 || board[i as usize] == 1 {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    empty_hex(canvas, 186 + (i % 13) * 61, 150 + j * 45, 30);
                } else if board[i as usize] == 2 {
                    mouse_hex(canvas, 186 + (i % 13) * 61, 150 + j * 45, 30);
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    block_hex(canvas, 186 + (i % 13) * 61, 150 + j * 45, 30);
                }
            }

        println!();
    }

    // Present the canvas
    canvas.present();

    Ok(())
}

fn print_board(board: [u8; 169]) {
    for i in board.iter().enumerate() {
        if i.0 % 13 == 0 {
            println!();
        }
        print!("{} ", board[i.0]);
    }
    println!();
}

fn empty_board() -> [u8; 169] {
    [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ]
}

fn main() -> Result<(), String> {
    println!("Starting Catch the mouse!");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Catch the mouse", 1000, 800)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build().expect("Canvas failed");

    let texture_creator = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"client/fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut event_pump = sdl_context.event_pump()?;
    let mut buttons = Buttons::new();

    render(&mut canvas, &texture_creator, &font, &buttons, empty_board())?;

    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            'running: loop {
                let mut sent = false;
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => {
                            stream.write_all("quiti".as_bytes()).unwrap();
                            break 'running;
                        }
                        Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            stream.write_all("quiti".as_bytes()).unwrap();
                            break 'running;
                        }
                        Event::MouseButtonDown { x, y, .. } => {
                            if buttons.host_join_visible
                                && (400..=600).contains(&x)
                                && (375..=425).contains(&y)
                            {
                                println!("Host button clicked");
                                buttons.host_join_visible = false;
                                buttons.computer_visible = true;
                                buttons.table = true;

                                stream.write_all("hosti".as_bytes()).unwrap();
                                println!("awaiting reply...");
                                sent = true;

                            } else if buttons.host_join_visible
                                && (400..=600).contains(&x)
                                && (455..=505).contains(&y)
                            {
                                println!("Join button clicked");
                                buttons.host_join_visible = false;
                                buttons.table = true;
                                buttons.refresh = true;
                                stream.write_all("joini".as_bytes()).unwrap();
                                println!("awaiting reply...");
                                sent = true;

                            } else if buttons.computer_visible
                                && (300..=500).contains(&x)
                                && (50..=100).contains(&y)
                            {
                                println!("Start button clicked");
                                buttons.computer_visible = false;
                                stream.write_all("start".as_bytes()).unwrap();
                                println!("awaiting reply...");
                                sent = true;
                            } else if buttons.refresh
                            && (300..=500).contains(&x)
                            && (50..=100).contains(&y)
                        {
                            println!("Refresh button clicked");
                            stream.write_all("refrs".as_bytes()).unwrap();
                            println!("awaiting reply...");
                            sent = true;
                        }
                            else if buttons.table && (130..=955).contains(&x) && (130..=710).contains(&y)
                            {
                                if buttons.computer_visible {
                                    buttons.computer_visible = false;
                                    buttons.refresh = true;
                                }
                                else {
                                    buttons.computer_visible = false;
                                }
                                let mut real_y = 0;
                                let mut real_x = 9;
                                let mut j = 0;
                                for i in 0..169 {
                                    if i % 13 == 0 && i > 0 {
                                        j += 1;
                                    }
                                    if j % 2 == 0 {
                                        let centerx = 160 + (i % 13) * 61;
                                        let centery = 150 + j * 45;
                                        if x > centerx - 30
                                            && x < centerx + 30
                                            && y > centery - 30
                                            && y < centery + 30
                                        {
                                            println!("i :{}, j: {}", i % 13, j);
                                            real_y = j;
                                            real_x = i % 13;
                                        }
                                    } else {
                                        let centerx = 186 + (i % 13) * 61;
                                        let centery = 150 + j * 45;
                                        if x > centerx - 30
                                            && x < centerx + 30
                                            && y > centery - 30
                                            && y < centery + 30
                                        {
                                            println!("i :{}, j: {}", i % 13, j);
                                            real_y = j;
                                            real_x = i % 13;
                                        }
                                    }
                                }
                                let mut yx = String::new();
                                if real_y > 9 {
                                    yx.push_str(&real_y.to_string());
                                } else {
                                    yx.push('0');
                                    yx.push_str(&real_y.to_string());
                                }
                                yx.push(' ');
                                if real_x > 9 {
                                    yx.push_str(&real_x.to_string());
                                } else {
                                    yx.push('0');
                                    yx.push_str(&real_x.to_string());
                                }

                                println!("{}", yx);
                                stream.write_all(yx.as_bytes()).unwrap();
                                println!("awaiting reply...");
                                buttons.computer_visible = false;
                                sent = true;
                            }
                        }
                        _ => {}
                    }
                }
                if sent {
                    let mut data: [u8; 169] = [0; 169];
                    match stream.read_exact(&mut data) {
                        Ok(_size) => {
                            if data[0] == 1 {
                                print_board(data);
                            }
                            else if data[0] == 3 { //win
                                buttons.host_join_visible = true;
                                buttons.computer_visible = false;
                                buttons.table = false;
                                buttons.winner = true;
                                buttons.refresh = false;
                            }
                            else if data[0] == 4 { //lose
                                buttons.host_join_visible = true;
                                buttons.computer_visible = false;
                                buttons.table = false;
                                buttons.loser = true;
                                buttons.refresh = false;
                            }
                            else if data[9] == 9 {
                                buttons.host_join_visible = true;
                                buttons.computer_visible = false;
                                buttons.table = false;
                                buttons.refresh = false;
                            }
                            else {
                                buttons.host_join_visible = true;
                                buttons.table = false;
                                buttons.computer_visible = false;
                            }
                            render(&mut canvas, &texture_creator, &font, &buttons, data)?;
                            buttons.winner = false;
                            buttons.loser = false;
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");

    Ok(())
}
