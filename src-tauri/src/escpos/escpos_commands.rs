pub struct EscposCommands;

impl EscposCommands {
    // Si no vas a usar ESC o GS fuera del impl, pueden quedarse sin 'pub'
    pub const ESC: u8 = 0x1B;
    pub const GS: u8 = 0x1D;

    // Se debe usar Self::ESC y Self::GS dentro del bloque impl
    pub const INICIALIZAR: [u8; 2] = [Self::ESC, 0x40];
    pub const CORTE_TOTAL: [u8; 3] = [Self::GS, 0x56, 0x00];
    pub const FEED_LINEAS: [u8; 3] = [Self::ESC, 0x64, 0x05];
    pub const CORTE_PARCIAL: [u8; 3] = [Self::GS, 0x56, 0x01];
    pub const NEGRITA_ON: [u8; 3] = [Self::ESC, 0x45, 0x01];
    pub const NEGRITA_OFF: [u8; 3] = [Self::ESC, 0x45, 0x00];
    pub const CENTRAR: [u8; 3] = [Self::ESC, 0x61, 0x01];
    pub const ALINEAR_IZQUIERDA: [u8; 3] = [Self::ESC, 0x61, 0x00];
    pub const TEXTO_GRANDE: [u8; 3] = [Self::GS, 0x21, 0x11];
    pub const TEXTO_NORMAL: [u8; 3] = [Self::GS, 0x21, 0x00];
    
}