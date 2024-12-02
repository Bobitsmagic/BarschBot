use std::collections::HashSet;

use piston::{Button, Event, Key, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, WindowSettings};
use piston_window::{graphics, Flip, G2dTexture, Glyphs, OpenGL, PistonWindow, Texture, TextureSettings};

use crate::{board::{piece_type::{ColoredPieceType, PieceType}, square::{self, Square}}, gui::render_state::ANIMATION_TIME, moves::chess_move::ChessMove};

use super::{engine_handle::EngineHandle, render_state::RenderState};

pub struct Visualizer {
    glyphs: Glyphs,
    latest_render_state: RenderState,
    input_state: HashSet<Button>,
    handle: EngineHandle,
    window: PistonWindow,
    textures: Vec<G2dTexture>,
    last_click_pos: [f64; 2],
    cursor_pos: [f64; 2],
}

const SIDE_LENGTH: u32 = 900;

impl Visualizer {
    pub fn new(engine_handle: EngineHandle) -> Self {
        let opengl = OpenGL::V3_2;
        let mut window: PistonWindow =
            WindowSettings::new("Chess stuff", [SIDE_LENGTH, SIDE_LENGTH])
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

        let assets_folder = "textures";

        const FILE_NAMES: [&str; 12] = [
            "wP.png", 
            "wN.png", 
            "wB.png", 
            "wR.png", 
            "wQ.png", 
            "wK.png",
            "bP.png", 
            "bN.png", 
            "bB.png", 
            "bR.png", 
            "bQ.png", 
            "bK.png"]; 

        let mut textures = Vec::new();

        for i in 0..FILE_NAMES.len() {
            let image = format!("{}/{}", assets_folder, FILE_NAMES[i]);
            let texture: G2dTexture = Texture::from_path(
                &mut window.create_texture_context(),
                &image,
                Flip::None,
                &TextureSettings::new()
            ).unwrap();

            textures.push(texture);
        }

        let glyphs: Glyphs = window.load_font(&format!("{}/font.ttf", assets_folder)).expect(&format!("Could not find font at"));
                
        return Visualizer { 
            glyphs,
            input_state: HashSet::new(),
            cursor_pos: [0.0, 0.0],
            last_click_pos: [0.0, 0.0],
            latest_render_state: RenderState::new(), 
            handle: engine_handle, 
            window, 
            textures };
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            if let Some(rs) = self.handle.recive_render_state() {
                self.latest_render_state = rs;
            }
            
            //Event handling
            if let Some(_) = e.render_args() {
                self.render_board(e);
            }
            else if let Some(button) = e.press_args() {
                match button {
                    Button::Mouse(MouseButton::Left) => {
                        self.last_click_pos = self.cursor_pos.clone()
                    }

                    _ => {
                        self.input_state.insert(button);
                    }
                }
            }
            else if let Some(button) = e.release_args() {
                match button {
                    Button::Mouse(MouseButton::Left) => {
                        println!("Handle move from {:?} to {:?}", self.last_click_pos, self.cursor_pos);

                        let width = self.window.window.window.inner_size().width;
                        let height = self.window.window.window.inner_size().height;

                        let start = transfrom_choords(self.last_click_pos, width, height);
                        let end = transfrom_choords(self.cursor_pos, width, height);

                        let mpt = self.latest_render_state.piece_board[start];
                        let cpt = self.latest_render_state.piece_board[end];

                        if mpt == ColoredPieceType::None {
                            continue;
                        }

                        let cm = if (end.rank() == 0 || end.rank() == 7) && mpt.is_pawn() {
                            println!("Promotion move!");

                            let pt = if self.input_state.contains(&Button::Keyboard(Key::R)) {
                                PieceType::Rook
                            }
                            else if self.input_state.contains(&Button::Keyboard(Key::B)) {
                                PieceType::Bishop
                            }
                            else if self.input_state.contains(&Button::Keyboard(Key::N)) {
                                PieceType::Knight
                            }
                            else {
                                PieceType::Queen
                            };
                            
                            let color = mpt.color();
                            let promotion_piece = pt.colored(color);


                            ChessMove::new_pawn(start, end, mpt, cpt, promotion_piece)
                        }
                        else {
                            ChessMove::new(start, end, mpt, cpt)
                        };

                        println!("Sending move:");
                        cm.print();
                        self.handle.send_move(cm);

                        fn transfrom_choords(pos: [f64; 2], width: u32, height: u32) -> i8 {
                            let x = pos[0] / width as f64 * 8.0;
                            let y = pos[1] / height as f64 * 8.0;
                            

                            return square::from_file_rank(x as i8, 7 - (y as i8));
                        }
                    }
                    _ => {
                        self.input_state.remove(&button);
                    }
                }
            } 
            else if let Some(pos) = e.mouse_cursor_args() {
                self.cursor_pos = pos;
            }
            else if let Some(args) = e.update_args() {
                self.latest_render_state.animation_time += args.dt;
            }
        }
    }

    pub fn render_board(&mut self, event: Event) -> bool {
        use graphics::*;
        
        const LIGHT_SQUARE: [f32; 4] = [240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0, 1.0];
        const DARK_SQUARE: [f32; 4] = [181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0, 1.0];
        const LIGHT_MOVE_SQUARE: [f32; 4] = [205.0 / 255.0, 210.0 / 255.0, 106.0 / 255.0, 1.0];
        const DARK_MOVE_SQUARE: [f32; 4] = [170.0 / 255.0, 162.0 / 255.0, 58.0 / 255.0, 1.0];
        const FILE_NAMES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
        
        let square_side_length = self.window.window.window.inner_size().width as f64 / 8.0;
        let window_height = self.window.window.window.inner_size().height as f64;
        let window_width = self.window.window.window.inner_size().width as f64;
        let square = rectangle::square(0.0, 0.0, square_side_length);

        let lm = self.latest_render_state.lm;
        let flip = self.latest_render_state.flip;
        let type_field = &self.latest_render_state.piece_board;

        let completion = (self.latest_render_state.animation_time / ANIMATION_TIME).min(1.0);
        let ratio = completion - 0.1 * (completion * std::f64::consts::PI * 2.0).sin();
        
        self.window.draw_2d(&event, |context, graphics, device| {
            clear(DARK_SQUARE, graphics);

            const SQUARE_FONT_SIZE: u32 = 20;
            const SQUARE_MARGIN: f64 = 5.0;

            for x in 0..8 {
                for y in 0..8 {
                    let sq = x + y * 8;

                    let mut transform = context
                    .transform
                        .trans(x as f64 * square_side_length, (7 - y)  as f64 * square_side_length);

                    if flip {
                        transform = context
                        .transform
                        .trans((7 - x) as f64 * square_side_length, y  as f64 * square_side_length);
                    }

                    if sq.is_light() {
                        rectangle(LIGHT_SQUARE, square, transform, graphics);
                    } 

                    
                    if !lm.is_null_move() {
                        if sq == lm.start || 
                        sq == lm.end {
                            if sq.is_light() {
                                rectangle(LIGHT_MOVE_SQUARE, square, transform, graphics);
                            }
                            else {
                                rectangle(DARK_MOVE_SQUARE, square, transform, graphics);
                            }
                        } 
                        
                        let tp = type_field[x + y * 8];
                        if tp != ColoredPieceType::None && sq != lm.end {
                            let texture = &self.textures[tp as usize];
                            image(texture, 
                                transform.scale(square_side_length / texture.get_width() as f64, square_side_length /texture.get_height() as f64), 
                                graphics);
                        }   
                    }
                    else {
                        let tp = type_field[x + y * 8];
                        if tp != ColoredPieceType::None {
                            let texture = &self.textures[tp as usize];
                            image(texture, 
                                transform.scale(square_side_length / texture.get_width() as f64, square_side_length /texture.get_height() as f64), 
                                graphics);
                        }
                    }


                }
            }


            //Draw file names
            for x in 0..8 {
                let text = format!("{}", FILE_NAMES[x]);        
                let color = if x % 2 == 0 { DARK_SQUARE } else { LIGHT_SQUARE };
                let position = x as f64 * square_side_length;

                text::Text::new_color(color, SQUARE_FONT_SIZE)
                .draw(
                    &text,
                    &mut self.glyphs,
                    &context.draw_state,
                    context.transform.trans(position + SQUARE_MARGIN, window_height - SQUARE_MARGIN),
                    graphics,
                )
                .unwrap();
            }
            //Draw rank names
            for y in 0..8 {
                let text = format!("{}", 8 - y);        
                let color = if y % 2 == 0 { DARK_SQUARE } else { LIGHT_SQUARE };
                let position = y as f64 * square_side_length;

                let text_width = self.glyphs.width(SQUARE_FONT_SIZE, &text).unwrap();

                text::Text::new_color(color, SQUARE_FONT_SIZE)
                .draw(
                    &text,
                    &mut self.glyphs,
                    &context.draw_state,
                    context.transform.trans(window_width - text_width - SQUARE_MARGIN, position + SQUARE_MARGIN + SQUARE_FONT_SIZE as f64),
                    graphics,
                ).unwrap();
            }

            if !lm.is_null_move() {
                let mx = lm.start.file() as f64 + 
                    (lm.end.file() as f64 - lm.start.file() as f64) * ratio;
            
                let my = lm.start.rank() as f64 + 
                    (lm.end.rank() as f64 - lm.start.rank() as f64) * ratio;

                let transform = context
                    .transform
                    .trans(mx * square_side_length, (7.0 - my) as f64 * square_side_length);

                
                let mut texture = &self.textures[lm.move_piece as usize];

                if lm.is_promotion() && ratio > 0.9 {
                    texture = &self.textures[lm.promotion_piece as usize];
                }

                image(texture, 
                    transform.scale(square_side_length / texture.get_width() as f64, square_side_length /texture.get_height() as f64), 
                    graphics);
            }

            self.glyphs.factory.encoder.flush(device);
        });

        return true;        
    }
}