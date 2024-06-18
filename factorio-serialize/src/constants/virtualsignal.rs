use enum_primitive_derive::Primitive;
use factorio_serialize_derive::ReplayReadWriteEnumU16;
use num_traits::{FromPrimitive, ToPrimitive};


// Version: 1.1.107
// Extraction method: util::export_prototypes
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Primitive, ReplayReadWriteEnumU16)]
pub enum VirtualSignal {
  SignalEverything = 1,
  SignalAnything = 2,
  SignalEach = 3,
  Signal0 = 4,
  Signal1 = 5,
  Signal2 = 6,
  Signal3 = 7,
  Signal4 = 8,
  Signal5 = 9,
  Signal6 = 10,
  Signal7 = 11,
  Signal8 = 12,
  Signal9 = 13,
  SignalA = 14,
  SignalB = 15,
  SignalC = 16,
  SignalD = 17,
  SignalE = 18,
  SignalF = 19,
  SignalG = 20,
  SignalH = 21,
  SignalI = 22,
  SignalJ = 23,
  SignalK = 24,
  SignalL = 25,
  SignalM = 26,
  SignalN = 27,
  SignalO = 28,
  SignalP = 29,
  SignalQ = 30,
  SignalR = 31,
  SignalS = 32,
  SignalT = 33,
  SignalU = 34,
  SignalV = 35,
  SignalW = 36,
  SignalX = 37,
  SignalY = 38,
  SignalZ = 39,
  SignalRed = 40,
  SignalGreen = 41,
  SignalBlue = 42,
  SignalYellow = 43,
  SignalPink = 44,
  SignalCyan = 45,
  SignalWhite = 46,
  SignalGrey = 47,
  SignalBlack = 48,
  SignalCheck = 49,
  SignalInfo = 50,
  SignalDot = 51,
  SignalUnknown = 52,
}
impl VirtualSignal {
  pub fn name(self) -> &'static str {
    match self {
      VirtualSignal::SignalEverything => "signal-everything",
      VirtualSignal::SignalAnything => "signal-anything",
      VirtualSignal::SignalEach => "signal-each",
      VirtualSignal::Signal0 => "signal-0",
      VirtualSignal::Signal1 => "signal-1",
      VirtualSignal::Signal2 => "signal-2",
      VirtualSignal::Signal3 => "signal-3",
      VirtualSignal::Signal4 => "signal-4",
      VirtualSignal::Signal5 => "signal-5",
      VirtualSignal::Signal6 => "signal-6",
      VirtualSignal::Signal7 => "signal-7",
      VirtualSignal::Signal8 => "signal-8",
      VirtualSignal::Signal9 => "signal-9",
      VirtualSignal::SignalA => "signal-A",
      VirtualSignal::SignalB => "signal-B",
      VirtualSignal::SignalC => "signal-C",
      VirtualSignal::SignalD => "signal-D",
      VirtualSignal::SignalE => "signal-E",
      VirtualSignal::SignalF => "signal-F",
      VirtualSignal::SignalG => "signal-G",
      VirtualSignal::SignalH => "signal-H",
      VirtualSignal::SignalI => "signal-I",
      VirtualSignal::SignalJ => "signal-J",
      VirtualSignal::SignalK => "signal-K",
      VirtualSignal::SignalL => "signal-L",
      VirtualSignal::SignalM => "signal-M",
      VirtualSignal::SignalN => "signal-N",
      VirtualSignal::SignalO => "signal-O",
      VirtualSignal::SignalP => "signal-P",
      VirtualSignal::SignalQ => "signal-Q",
      VirtualSignal::SignalR => "signal-R",
      VirtualSignal::SignalS => "signal-S",
      VirtualSignal::SignalT => "signal-T",
      VirtualSignal::SignalU => "signal-U",
      VirtualSignal::SignalV => "signal-V",
      VirtualSignal::SignalW => "signal-W",
      VirtualSignal::SignalX => "signal-X",
      VirtualSignal::SignalY => "signal-Y",
      VirtualSignal::SignalZ => "signal-Z",
      VirtualSignal::SignalRed => "signal-red",
      VirtualSignal::SignalGreen => "signal-green",
      VirtualSignal::SignalBlue => "signal-blue",
      VirtualSignal::SignalYellow => "signal-yellow",
      VirtualSignal::SignalPink => "signal-pink",
      VirtualSignal::SignalCyan => "signal-cyan",
      VirtualSignal::SignalWhite => "signal-white",
      VirtualSignal::SignalGrey => "signal-grey",
      VirtualSignal::SignalBlack => "signal-black",
      VirtualSignal::SignalCheck => "signal-check",
      VirtualSignal::SignalInfo => "signal-info",
      VirtualSignal::SignalDot => "signal-dot",
      VirtualSignal::SignalUnknown => "signal-unknown",
    }
  }
  pub fn from_name(name: &str) -> VirtualSignal {
    match name {
      "signal-everything" => VirtualSignal::SignalEverything,
      "signal-anything" => VirtualSignal::SignalAnything,
      "signal-each" => VirtualSignal::SignalEach,
      "signal-0" => VirtualSignal::Signal0,
      "signal-1" => VirtualSignal::Signal1,
      "signal-2" => VirtualSignal::Signal2,
      "signal-3" => VirtualSignal::Signal3,
      "signal-4" => VirtualSignal::Signal4,
      "signal-5" => VirtualSignal::Signal5,
      "signal-6" => VirtualSignal::Signal6,
      "signal-7" => VirtualSignal::Signal7,
      "signal-8" => VirtualSignal::Signal8,
      "signal-9" => VirtualSignal::Signal9,
      "signal-A" => VirtualSignal::SignalA,
      "signal-B" => VirtualSignal::SignalB,
      "signal-C" => VirtualSignal::SignalC,
      "signal-D" => VirtualSignal::SignalD,
      "signal-E" => VirtualSignal::SignalE,
      "signal-F" => VirtualSignal::SignalF,
      "signal-G" => VirtualSignal::SignalG,
      "signal-H" => VirtualSignal::SignalH,
      "signal-I" => VirtualSignal::SignalI,
      "signal-J" => VirtualSignal::SignalJ,
      "signal-K" => VirtualSignal::SignalK,
      "signal-L" => VirtualSignal::SignalL,
      "signal-M" => VirtualSignal::SignalM,
      "signal-N" => VirtualSignal::SignalN,
      "signal-O" => VirtualSignal::SignalO,
      "signal-P" => VirtualSignal::SignalP,
      "signal-Q" => VirtualSignal::SignalQ,
      "signal-R" => VirtualSignal::SignalR,
      "signal-S" => VirtualSignal::SignalS,
      "signal-T" => VirtualSignal::SignalT,
      "signal-U" => VirtualSignal::SignalU,
      "signal-V" => VirtualSignal::SignalV,
      "signal-W" => VirtualSignal::SignalW,
      "signal-X" => VirtualSignal::SignalX,
      "signal-Y" => VirtualSignal::SignalY,
      "signal-Z" => VirtualSignal::SignalZ,
      "signal-red" => VirtualSignal::SignalRed,
      "signal-green" => VirtualSignal::SignalGreen,
      "signal-blue" => VirtualSignal::SignalBlue,
      "signal-yellow" => VirtualSignal::SignalYellow,
      "signal-pink" => VirtualSignal::SignalPink,
      "signal-cyan" => VirtualSignal::SignalCyan,
      "signal-white" => VirtualSignal::SignalWhite,
      "signal-grey" => VirtualSignal::SignalGrey,
      "signal-black" => VirtualSignal::SignalBlack,
      "signal-check" => VirtualSignal::SignalCheck,
      "signal-info" => VirtualSignal::SignalInfo,
      "signal-dot" => VirtualSignal::SignalDot,
      "signal-unknown" => VirtualSignal::SignalUnknown,
      name => panic!("unknown VirtualSignal \"{name}\""),
    }
  }
}