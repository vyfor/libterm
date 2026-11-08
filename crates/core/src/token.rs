/// https://en.wikipedia.org/wiki/ANSI_escape_code#Fe_Escape_sequences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenRef<'a> {
    Text(&'a str),
    C0(u8),
    Esc {
        intermediate: Option<u8>,
        final_byte: u8,
    },
    Ss3(u8),
    Csi {
        prefix: Option<u8>,
        params: &'a [u8],
        intermediate: Option<u8>,
        final_byte: u8,
    },
    Osc(&'a [u8]),
    Dcs(&'a [u8]),
    Apc(&'a [u8]),
    Pm(&'a [u8]),
    Sos(&'a [u8]),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Text(String),
    C0(u8),
    Esc {
        intermediate: Option<u8>,
        final_byte: u8,
    },
    Ss3(u8),
    Csi {
        prefix: Option<u8>,
        params: Vec<u8>,
        intermediate: Option<u8>,
        final_byte: u8,
    },
    Osc(Vec<u8>),
    Dcs(Vec<u8>),
    Apc(Vec<u8>),
    Pm(Vec<u8>),
    Sos(Vec<u8>),
}

impl<'a> TokenRef<'a> {
    pub fn to_owned(&self) -> Token {
        match self {
            TokenRef::Text(s) => Token::Text((*s).to_string()),
            TokenRef::C0(b) => Token::C0(*b),
            TokenRef::Esc {
                intermediate,
                final_byte,
            } => Token::Esc {
                intermediate: *intermediate,
                final_byte: *final_byte,
            },
            TokenRef::Ss3(b) => Token::Ss3(*b),
            TokenRef::Csi {
                prefix,
                params,
                intermediate,
                final_byte,
            } => Token::Csi {
                prefix: *prefix,
                params: params.to_vec(),
                intermediate: *intermediate,
                final_byte: *final_byte,
            },
            TokenRef::Osc(b) => Token::Osc(b.to_vec()),
            TokenRef::Dcs(b) => Token::Dcs(b.to_vec()),
            TokenRef::Apc(b) => Token::Apc(b.to_vec()),
            TokenRef::Pm(b) => Token::Pm(b.to_vec()),
            TokenRef::Sos(b) => Token::Sos(b.to_vec()),
        }
    }
}
