pub struct Cpf([u8; Self::SIZE]);

#[derive(Debug, Clone, Copy)]
pub enum ParseCpfError {
    WrongLength,
    NonNumeric,
}

impl Cpf {
    const SIZE: usize = 11;
}
