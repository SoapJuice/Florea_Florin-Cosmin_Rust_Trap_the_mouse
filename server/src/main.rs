use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::thread::{self};

#[derive(Debug)]
struct Lobby {
    player1: std::net::SocketAddr,
    player2: std::net::SocketAddr,
    board: [u8; 169],
    turn: u8,
    win: u8,
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

fn host(lobbys: &mut Vec<Lobby>, addr: std::net::SocketAddr) {
    let empty = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0000);
    lobbys.push(Lobby {
        player1: addr,
        player2: empty,
        board: empty_board(),
        turn: 1,
        win: 0,
    });
}

fn check_win(lobbys: &mut Vec<Lobby>, addr: std::net::SocketAddr) -> u8 {
    for i in lobbys {
        if i.player1 == addr {
            if i.win == 1 {
                return 1;
            } else if i.win == 2 {
                return 2;
            } else {
                return 0;
            }
        } else if i.player2 == addr {
            if i.win == 1 {
                return 2;
            } else if i.win == 2 {
                return 1;
            } else {
                return 0;
            }
        }
    }
    2
}

fn mouse_block(board: [u8; 169]) -> bool {
    for i in board.iter().enumerate() {
        if *i.1 == 2 {
            println!("i {}, poz {}, prox: {}",i.0,*i.1,around(i.0, 0, board));
            return !(around(i.0, 0, board) || around(i.0, 1, board));
        }
    }
    true
}

fn around(s: usize, m: u8, board:[u8;169]) -> bool {
    println!("{}",s);
    if (s / 13) % 2 == 1 {
        println!("{}, {}",s / 13, m);
        m == board[s + 13] || m == board[s - 13 ]|| m == board[s + 1] || m == board[s - 1] || m == board[s - 12] || m == board[s + 14]
    }
    else {
        m == board[s + 13] || m == board[s - 13] || m == board[s + 1] || m == board[s - 1] || m == board[s - 14] || m == board[s + 12]
    }
}

fn valid_move(s: i8, m: i8) -> bool {
    println!("{}",s);
    if (s / 13) % 2 == 1 {
        println!("{}, {}",s / 13, m);
        m == s + 13 || m == s - 13 || m == s + 1 || m == s - 1 || m == s - 12 || m == s + 14
    }
    else {
        m == s + 13 || m == s - 13 || m == s + 1 || m == s - 1 || m == s - 14 || m == s + 12
    }
}

fn join(lobbys: &mut Vec<Lobby>, addr: std::net::SocketAddr) -> bool {
    let empty = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0000);
    for i in lobbys {
        if i.player2 == empty {
            i.player2 = addr;
            return true;
        }
        println!("lobbys: {:?}", i);
    }
    false
}

fn game(
    lobbys: &mut Vec<Lobby>,
    addr: std::net::SocketAddr,
    player: u8,
    data: &str, computer: bool,
) -> [u8; 169] {
    let mut number1= 0;
    let mut number2 = 0;
    for i in data.chars() {
        if i.is_numeric() {
            number2 = number2 * 10 + (i as u8 - b'0');
        }
        else {
            number1 = number2;
            number2 = 0;
        }
    }
    if number1 < 1 && number1 > 11 && number2 < 1 && number2 > 11 {
        return empty_board();
    }
    println!("row: {}, col: {}", number1, number2); 

    for i in lobbys {
        println!("{:?}", i);
        if i.player1 == addr || i.player2 == addr {
            println!("Am gasit lobby-ul!");
            if player == 1 && i.turn == 1 {
                println!("Am gasit lobby-ul ca jucatorul host!");
                let index = number1 * 13 + number2;
                if i.board[index as usize] == 0 {
                    i.board[index as usize] = 3;
                    i.turn = 2;
                    if mouse_block(i.board) {
                        println!("Am blocat soarecele!");
                        i.win = 1;
                    }
                    return i.board;
                }
            } else if player == 2 && i.turn == 2 {
                println!("Am gasit lobby-ul ca jucatorul join!");
                    for j in 0..169 {
                        if i.board[j] == 2 {
                            let index = number1 * 13 + number2;
                            if i.board[index as usize] == 0 && valid_move(j as i8, index as i8) {
                                i.board[j] = 0;
                                i.board[index as usize] = 2;
                                i.turn = 1;
                                return i.board;
                            } else if i.board[index as usize] == 1 && valid_move(j as i8, index as i8) {
                                i.board[j] = 0;
                                i.board[index as usize] = 2;
                                i.turn = 1;
                                println!("Am evadat!");
                                i.win = 2;
                                return i.board;
                            }
                        }
                    }
                    
            }
                else if computer &&  i.turn == 2  {
                    println!("Am gasit lobby-ul ca robotul!");
                    for j in 0..169 {
                        if i.board[j] == 2 {
                            println!("mouse possition: {}",j);
                            let decision = make_move(&mut i.board);
                            println!("{}", decision);
                            println!("board de dupa make move");
                            if decision == 2 {
                                i.win = 2;
                            }
                            else if decision == 1 {
                                i.win = 1;
                            }
                            i.turn = 1;
                            return i.board;
                        }
                        
                    }
                }
            return i.board;
        }
    }
    println!("exit");
    empty_board()
}

fn get_lobby(lobbys: &mut Vec<Lobby>, addr: std::net::SocketAddr) -> [u8; 169] {
    for i in lobbys {
        if i.player1 == addr || i.player2 == addr {
            return i.board;
        }
    }
    let mut menu = empty_board();
    menu[0] = 9;
    menu
}

fn remove_lobby(lobbys: &mut Vec<Lobby>, addr: std::net::SocketAddr) {
    lobbys.retain(|i| i.player1 != addr && i.player2 != addr);
}

fn make_move(board: &mut [u8; 169]) -> u8 {
    for i in 0..169 {
        if board[i] == 2 {
            println!("make move i: {}", i);
            if board[i + 1] == 1 {
                board[i + 1] = 2;
                board[i] = 0;
                return 2;
            } else if board[i - 1] == 1 {
                board[i - 1] = 2;
                board[i] = 0;
                return 2;
            } else if (i / 13) % 2 == 0 {
                if board[i + 13] == 1 {
                    board[i + 13] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i - 13] == 1 {
                    board[i - 13] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i - 14] == 1 {
                    board[i - 14] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i + 12] == 1 {
                    board[i + 12] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i + 1] == 0 {
                    board[i + 1] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i - 1] == 0 {
                    board[i - 1] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i + 13] == 0 {
                    board[i + 13] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i - 13] == 0 {
                    board[i - 13] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i - 14] == 0 {
                    board[i - 14] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i + 12] == 0 {
                    board[i + 12] = 2;
                    board[i] = 0;
                    return 0;
                } else {
                    return 1;
                }
            } else if board[i + 13] == 1 {
                    board[i + 13] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i - 13] == 1 {
                    board[i - 13] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i + 14] == 1 {
                    board[i + 14] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i - 12] == 1 {
                    board[i - 12] = 2;
                    board[i] = 0;
                    return 2;
                } else if board[i + 1] == 0 {
                    board[i + 1] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i - 1] == 0 {
                    board[i - 1] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i + 13] == 0 {
                    board[i + 13] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i - 13] == 0 {
                    board[i - 13] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i + 14] == 0 {
                    board[i + 14] = 2;
                    board[i] = 0;
                    return 0;
                } else if board[i - 12] == 0 {
                    board[i - 12] = 2;
                    board[i] = 0;
                    return 0;
                } else {
                    return 1;
                }
             
        }
    }
    1
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

fn main_menu(mut stream: TcpStream, lobbys: Arc<Mutex<Vec<Lobby>>>) {
    let mut computer = false;
    let mut player = 0;
    let mut data = [0_u8; 5];
    loop {
        match stream.read_exact(&mut data) {
            Ok(_size) => {
                let msg = from_utf8(&data).ok().unwrap();
                println!("{}",msg);
                if msg == "hosti" && player == 0 {
                    println!("am intrat in host");
                    player = 1;
                    host(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                    let board_response = empty_board();
                    println!("{:?}", board_response);
                    stream.write(&board_response).ok().unwrap_or_default();
                    println!("{}", player);
                } else if msg == "joini" && player == 0 {
                    println!("am intrat in join");
                    player = 2;
                    let mut board_response = empty_board();

                    if join(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap()) {
                        println!("{:?}", board_response);
                        stream.write(&board_response).ok().unwrap_or_default();
                    } else {
                        board_response[0] = 7; 
                        stream.write(&board_response).ok().unwrap_or_default();
                    }
                } else if player == 1 && msg == "start" {
                    println!("am intrat in start");
                    computer = true;
                    let mut board_response = empty_board();
                    let computer_addr =
                        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 1111);
                    if join(&mut lobbys.lock().unwrap(), computer_addr) {
                        println!("{:?}", board_response);
                        stream.write(&board_response).ok().unwrap_or_default();
                    } else {
                        board_response[0] = 7;
                        stream.write(&board_response).ok().unwrap_or_default();
                    }
                    println!("{}", computer);
                } else if msg == "quiti" {
                    remove_lobby(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                    break;
                } 
                else if msg == "refrs" {
                    println!("am intrat in refrs");
                    let mut board_response = get_lobby(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                    let winning = check_win(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                        if  winning == 1 { //win
                            board_response[0] = 3;
                            player = 0;
                            computer = false;
                            remove_lobby(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                            stream.write(&board_response).ok().unwrap_or_default();
                        }
                        else if winning == 2 { //lost
                            board_response[0] = 4;
                            player = 0;
                            computer = false;
                            remove_lobby(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                            stream.write(&board_response).ok().unwrap_or_default();
                        }
                        else {
                            stream.write(&board_response).ok().unwrap_or_default();
                        }
                }
                
                else if player != 0 {
                    println!("am intrat in move");
                    let mut board_response = game(
                        &mut lobbys.lock().unwrap(),
                        stream.peer_addr().unwrap(),
                        player,
                        msg, computer, 
                    );
                    println!("board de dupa game");
                    print_board(board_response);
                    if board_response == empty_board() {
                        board_response[0] = 7;
                        println!(
                            "Nu am gasit jucatorul cu adresa: {}",
                            stream.peer_addr().unwrap()
                        );
                    }
                    else {
                        let winning = check_win(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                        if  winning == 1 { //win
                            board_response[0] = 3;
                            player = 0;
                            computer = false;
                            remove_lobby(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                            stream.write(&board_response).ok().unwrap_or_default();
                        }
                        else if winning == 2 { //lost
                            board_response[0] = 4;
                            player = 0;
                            computer = false;
                            remove_lobby(&mut lobbys.lock().unwrap(), stream.peer_addr().unwrap());
                            stream.write(&board_response).ok().unwrap_or_default();
                        }
                        else {
                            stream.write(&board_response).ok().unwrap_or_default();
                        }
                        // if player != 0 && computer == false {
                        //     while check(player, &mut lobbys.lock().unwrap(), stream.peer_addr().unwrap()) {
                        //         sleep(Duration::from_secs(3));
                        //     }
                        //     stream.write(&board_response).ok().unwrap_or_default();
                        // }
                    }
                }
                else {
                    stream.write(&empty_board()).ok().unwrap_or_default();
                }
            }
            Err(_) => {
                println!(
                    "A aparut o eroare la adresa: {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
    println!("Am iesit din Thread!");
}

fn main() {
    let lobbys: Arc<Mutex<Vec<Lobby>>> = Arc::new(Mutex::new(Vec::new()));
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        println!("test de ceva");
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let lobbys_clone = Arc::clone(&lobbys);
                thread::spawn(move || {
                    main_menu(stream, lobbys_clone);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}
