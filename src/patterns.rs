use note::Note;

// Each instrument has a 16 note pattern
struct InstrumentPattern([bool; 16]);

// For simplicity, assume the drum machine has 4 instruments (kick, snare, closed hat, open hat)
pub struct MachinePattern([InstrumentPattern; 4]);

// drum note numbers
const KICK_NOTE: u8 = 36;
const SNARE_NOTE: u8 = 38;
const CH_NOTE: u8 = 42;
const OH_NOTE: u8 = 46;

const VELOCITY: u8 = 100;

impl InstrumentPattern {
    // These could be a straightforward mapping of bit positions to pattern positions.
    // However, to generate more common variations first, I'm going to map the lowest bits to the
    // 1/4s, then the 1/8s between them, then finally the reminaing 1/16ths. The indiviual
    // instruments have been set up so that pattern 0 is the Amen break.
    fn kick_from_u16(num: u16) -> Self {
        InstrumentPattern ([
            ((num & 0x8000) == 0),  // 1
            ((num & 0x0010) != 0),
            ((num & 0x0001) == 0),  // and
            ((num & 0x0020) != 0),
            ((num & 0x2000) != 0),  // 2
            ((num & 0x0040) != 0),
            ((num & 0x0002) != 0),  // and
            ((num & 0x0080) != 0),
            ((num & 0x4000) != 0),  // 3
            ((num & 0x0100) != 0),
            ((num & 0x0004) == 0),  // and
            ((num & 0x0200) == 0),
            ((num & 0x1000) != 0),  // 4
            ((num & 0x0400) != 0),
            ((num & 0x0008) != 0),  // and
            ((num & 0x0800) != 0),
        ])
    }

    fn snare_from_u16(num: u16) -> Self {
        InstrumentPattern ([
            ((num & 0x0001) != 0),  // 1
            ((num & 0x0100) != 0),
            ((num & 0x0010) != 0),  // and
            ((num & 0x0200) != 0),
            ((num & 0x0002) == 0),  // 2
            ((num & 0x0400) != 0),
            ((num & 0x0020) != 0),  // and
            ((num & 0x0800) == 0),
            ((num & 0x0004) != 0),  // 3
            ((num & 0x1000) == 0),
            ((num & 0x0040) != 0),  // and
            ((num & 0x2000) != 0),
            ((num & 0x0008) == 0),  // 4
            ((num & 0x4000) != 0),
            ((num & 0x0080) != 0),  // and
            ((num & 0x8000) != 0),
        ])
    }

    fn ch_from_u16(num: u16) -> Self {
        InstrumentPattern ([
            ((num & 0x0001) != 0),  // 1
            ((num & 0x0100) == 0),
            ((num & 0x0010) == 0),  // and
            ((num & 0x0200) == 0),
            ((num & 0x0002) == 0),  // 2
            ((num & 0x0400) == 0),
            ((num & 0x0020) == 0),  // and
            ((num & 0x0800) == 0),
            ((num & 0x0004) == 0),  // 3
            ((num & 0x1000) == 0),
            ((num & 0x0040) == 0),  // and
            ((num & 0x2000) == 0),
            ((num & 0x0008) != 0),  // 4
            ((num & 0x4000) == 0),
            ((num & 0x0080) == 0),  // and
            ((num & 0x8000) == 0),
        ])
    }

    fn oh_from_u16(num: u16) -> Self {
        InstrumentPattern ([
            ((num & 0x0001) == 0),  // 1
            ((num & 0x0100) != 0),
            ((num & 0x0010) != 0),  // and
            ((num & 0x0200) != 0),
            ((num & 0x0002) != 0),  // 2
            ((num & 0x0400) != 0),
            ((num & 0x0020) != 0),  // and
            ((num & 0x0800) != 0),
            ((num & 0x0004) != 0),  // 3
            ((num & 0x1000) != 0),
            ((num & 0x0040) != 0),  // and
            ((num & 0x2000) != 0),
            ((num & 0x0008) == 0),  // 4
            ((num & 0x4000) != 0),
            ((num & 0x0080) != 0),  // and
            ((num & 0x8000) != 0),
        ])
    }
}

impl MachinePattern {
    pub fn from_u64(num: u64) -> Self {
        MachinePattern ([
            InstrumentPattern::kick_from_u16 (((num & 0xffff000000000000) >> 48) as u16),
            InstrumentPattern::snare_from_u16(((num & 0xffff00000000)     >> 32) as u16),
            InstrumentPattern::ch_from_u16   (((num & 0xffff0000)         >> 16) as u16),
            InstrumentPattern::oh_from_u16   ( (num & 0xffff)                    as u16),
        ])
    }

    pub fn step_iterator<'a>(&'a self) -> Box<Iterator<Item=Vec<Note>> + 'a> {
        Box::new(
            // zip the four patterns together to get an iterator giveing
            // the status of the 4 notes at each step
            self.0[0].0.iter()
            .zip(self.0[1].0.iter())
            .zip(self.0[2].0.iter())
            .zip(self.0[3].0.iter())
            .map(|(((a, b), c), d)| (a, b, c, d))
            // map the four note statuses to midi note values
            .map(|(a, b, c, d)| {
                let mut notes = Vec::with_capacity(4);
                if *a { notes.push(Note::new(KICK_NOTE, VELOCITY)) }
                if *b { notes.push(Note::new(SNARE_NOTE, VELOCITY)) }
                if *c { notes.push(Note::new(CH_NOTE, VELOCITY)) }
                if *d { notes.push(Note::new(OH_NOTE, VELOCITY)) }
                notes
            })
        )
    }
}
