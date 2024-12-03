use std::fmt::Display;

#[derive(Debug)]
pub enum CurrencyType {
    CAD,
    USD,
}

#[derive(Debug)]
pub enum ProcessingCentre {
    Halifax,
    Montreal,
    Toronto,
    Regina,
    Winnipeg,
    Calgary,
    Vancouver,
}

#[derive(Clone, Copy)]
pub enum RecordType {
    Header,
    Credit,
    Debit,
    Trailer,
}

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordType::Header => write!(f, "{}", 'A'),
            RecordType::Credit => write!(f, "{}", 'C'),
            RecordType::Debit => write!(f, "{}", 'D'),
            RecordType::Trailer => write!(f, "{}", 'Z'),
        }
    }
}
