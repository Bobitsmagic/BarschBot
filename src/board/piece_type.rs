use super::player_color::PlayerColor;

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
    pub fn colored(self, color: PlayerColor) -> ColoredPieceType {
        match color {
            PlayerColor::White => match self {
                PieceType::Pawn =>      ColoredPieceType::WhitePawn,
                PieceType::Knight =>    ColoredPieceType::WhiteKnight,
                PieceType::Bishop =>    ColoredPieceType::WhiteBishop,
                PieceType::Rook =>      ColoredPieceType::WhiteRook,
                PieceType::Queen =>     ColoredPieceType::WhiteQueen,
                PieceType::King =>      ColoredPieceType::WhiteKing,
                PieceType::None =>      ColoredPieceType::None,
            },
            PlayerColor::Black => match self {
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
    pub fn from_char(c: char) -> ColoredPieceType {
        match c {
            'P' => ColoredPieceType::WhitePawn,
            'N' => ColoredPieceType::WhiteKnight,
            'B' => ColoredPieceType::WhiteBishop,
            'R' => ColoredPieceType::WhiteRook,
            'Q' => ColoredPieceType::WhiteQueen,
            'K' => ColoredPieceType::WhiteKing,

            'p' => ColoredPieceType::BlackPawn,
            'n' => ColoredPieceType::BlackKnight,
            'b' => ColoredPieceType::BlackBishop,
            'r' => ColoredPieceType::BlackRook,
            'q' => ColoredPieceType::BlackQueen,
            'k' => ColoredPieceType::BlackKing,

            _ => ColoredPieceType::None,
        }
    }

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
            ColoredPieceType::None => " ",
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

    pub fn black(&self) -> ColoredPieceType {
        if *self == ColoredPieceType::None {
            return ColoredPieceType::None;
        }

        self.piece_type().colored(PlayerColor::Black)
    }

    pub fn white(&self) -> ColoredPieceType {
        if *self == ColoredPieceType::None {
            return ColoredPieceType::None;
        }

        self.piece_type().colored(PlayerColor::White)
    }

    pub fn color(&self) -> PlayerColor {
        match self {
            ColoredPieceType::WhitePawn => PlayerColor::White,
            ColoredPieceType::WhiteKnight => PlayerColor::White,
            ColoredPieceType::WhiteBishop => PlayerColor::White,
            ColoredPieceType::WhiteRook => PlayerColor::White,
            ColoredPieceType::WhiteQueen => PlayerColor::White,
            ColoredPieceType::WhiteKing => PlayerColor::White,

            ColoredPieceType::BlackPawn => PlayerColor::Black,
            ColoredPieceType::BlackKnight => PlayerColor::Black,
            ColoredPieceType::BlackBishop => PlayerColor::Black,
            ColoredPieceType::BlackRook => PlayerColor::Black,
            ColoredPieceType::BlackQueen => PlayerColor::Black,
            ColoredPieceType::BlackKing => PlayerColor::Black,

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