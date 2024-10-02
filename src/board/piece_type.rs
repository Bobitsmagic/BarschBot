use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColoredPieceType {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,

    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,

    None,
}

pub const ALL_PIECE_TYPES: [PieceType; 7] = [
    PieceType::Pawn,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Rook,
    PieceType::Queen,
    PieceType::King,

    PieceType::None,
];

pub const ALL_COLORED_PIECE_TYPES: [ColoredPieceType; 13] = [
    ColoredPieceType::WhitePawn,
    ColoredPieceType::WhiteKnight,
    ColoredPieceType::WhiteBishop,
    ColoredPieceType::WhiteRook,
    ColoredPieceType::WhiteQueen,
    ColoredPieceType::WhiteKing,

    ColoredPieceType::BlackPawn,
    ColoredPieceType::BlackKnight,
    ColoredPieceType::BlackBishop,
    ColoredPieceType::BlackRook,
    ColoredPieceType::BlackQueen,
    ColoredPieceType::BlackKing,

    ColoredPieceType::None,
];

impl PieceType {
    pub fn colored(self, color: Color) -> ColoredPieceType {
        match color {
            Color::White => match self {
                PieceType::Pawn =>      ColoredPieceType::WhitePawn,
                PieceType::Knight =>    ColoredPieceType::WhiteKnight,
                PieceType::Bishop =>    ColoredPieceType::WhiteBishop,
                PieceType::Rook =>      ColoredPieceType::WhiteRook,
                PieceType::Queen =>     ColoredPieceType::WhiteQueen,
                PieceType::King =>      ColoredPieceType::WhiteKing,
                PieceType::None =>      ColoredPieceType::None,
            },
            Color::Black => match self {
                PieceType::Pawn =>      ColoredPieceType::BlackPawn,
                PieceType::Knight =>    ColoredPieceType::BlackKnight,
                PieceType::Bishop =>    ColoredPieceType::BlackBishop,
                PieceType::Rook =>      ColoredPieceType::BlackRook,
                PieceType::Queen =>     ColoredPieceType::BlackQueen,
                PieceType::King =>      ColoredPieceType::BlackKing,
                PieceType::None =>      ColoredPieceType::None,
            },
        }
    }

    pub fn is_orthogonal_slider(&self) -> bool {
        match self {
            PieceType::Rook | PieceType::Queen => true,
            _ => false,
        }
    }

    pub fn is_diagonal_slider(&self) -> bool {
        match self {
            PieceType::Bishop | PieceType::Queen => true,
            _ => false,
        }
    }

    pub fn is_slider(&self) -> bool {
        self.is_orthogonal_slider() || self.is_diagonal_slider()
    }
}

impl ColoredPieceType {
    pub fn to_char(&self) -> char {
        match self {
            ColoredPieceType::WhitePawn => 'P',
            ColoredPieceType::WhiteKnight => 'N',
            ColoredPieceType::WhiteBishop => 'B',
            ColoredPieceType::WhiteRook => 'R',
            ColoredPieceType::WhiteQueen => 'Q',
            ColoredPieceType::WhiteKing => 'K',

            ColoredPieceType::BlackPawn => 'p',
            ColoredPieceType::BlackKnight => 'n',
            ColoredPieceType::BlackBishop => 'b',
            ColoredPieceType::BlackRook => 'r',
            ColoredPieceType::BlackQueen => 'q',
            ColoredPieceType::BlackKing => 'k',

            ColoredPieceType::None => '.',
        }
    }

    pub fn to_symbol(&self) -> &'static str {
        match self {
            ColoredPieceType::WhitePawn => "♟",
            ColoredPieceType::WhiteKnight => "♞",
            ColoredPieceType::WhiteBishop => "♝",
            ColoredPieceType::WhiteRook => "♜",
            ColoredPieceType::WhiteQueen => "♛",
            ColoredPieceType::WhiteKing => "♚",

            ColoredPieceType::BlackPawn => "♙",
            ColoredPieceType::BlackKnight => "♘",
            ColoredPieceType::BlackBishop => "♗",
            ColoredPieceType::BlackRook => "♖",
            ColoredPieceType::BlackQueen => "♕",
            ColoredPieceType::BlackKing => "♔",
            ColoredPieceType::None => "□",
        }
    }

    pub fn piece_type(&self) -> PieceType {
        match self {
            ColoredPieceType::WhitePawn => PieceType::Pawn,
            ColoredPieceType::WhiteKnight => PieceType::Knight,
            ColoredPieceType::WhiteBishop => PieceType::Bishop,
            ColoredPieceType::WhiteRook => PieceType::Rook,
            ColoredPieceType::WhiteQueen => PieceType::Queen,
            ColoredPieceType::WhiteKing => PieceType::King,

            ColoredPieceType::BlackPawn => PieceType::Pawn,
            ColoredPieceType::BlackKnight => PieceType::Knight,
            ColoredPieceType::BlackBishop => PieceType::Bishop,
            ColoredPieceType::BlackRook => PieceType::Rook,
            ColoredPieceType::BlackQueen => PieceType::Queen,
            ColoredPieceType::BlackKing => PieceType::King,

            ColoredPieceType::None => PieceType::None,
        }
    }

    pub fn opposite(&self) -> ColoredPieceType {
        self.piece_type().colored(!self.color())
    }
    pub fn color(&self) -> Color {
        match self {
            ColoredPieceType::WhitePawn => Color::White,
            ColoredPieceType::WhiteKnight => Color::White,
            ColoredPieceType::WhiteBishop => Color::White,
            ColoredPieceType::WhiteRook => Color::White,
            ColoredPieceType::WhiteQueen => Color::White,
            ColoredPieceType::WhiteKing => Color::White,

            ColoredPieceType::BlackPawn => Color::Black,
            ColoredPieceType::BlackKnight => Color::Black,
            ColoredPieceType::BlackBishop => Color::Black,
            ColoredPieceType::BlackRook => Color::Black,
            ColoredPieceType::BlackQueen => Color::Black,
            ColoredPieceType::BlackKing => Color::Black,

            ColoredPieceType::None => panic!("Invalid piece type: None"),
        }
    }

    pub fn is_none(&self) -> bool {
        *self == ColoredPieceType::None
    }
    
    pub fn is_pawn(&self) -> bool {
        match self {
            ColoredPieceType::WhitePawn | ColoredPieceType::BlackPawn => true,
            _ => false,
        } 
    }

    pub fn is_knight(&self) -> bool {
        match self {
            ColoredPieceType::WhiteKnight | ColoredPieceType::BlackKnight => true,
            _ => false,
        } 
    }

    pub fn is_bishop(&self) -> bool {
        match self {
            ColoredPieceType::WhiteBishop | ColoredPieceType::BlackBishop => true,
            _ => false,
        } 
    }

    pub fn is_rook(&self) -> bool {
        match self {
            ColoredPieceType::WhiteRook | ColoredPieceType::BlackRook => true,
            _ => false,
        } 
    }

    pub fn is_queen(&self) -> bool {
        match self {
            ColoredPieceType::WhiteQueen | ColoredPieceType::BlackQueen => true,
            _ => false,
        } 
    }

    pub fn is_king(&self) -> bool {
        match self {
            ColoredPieceType::WhiteKing | ColoredPieceType::BlackKing => true,
            _ => false,
        } 
    }

    pub fn is_orthogonal_slider(&self) -> bool {
        self.piece_type().is_orthogonal_slider()
    }
    pub fn is_diagonal_slider(&self) -> bool {
        self.piece_type().is_diagonal_slider()
    }
    pub fn is_slider(&self) -> bool {
        self.piece_type().is_slider()
    }
}