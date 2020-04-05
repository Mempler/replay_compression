use half::f16;

#[derive(Debug, Clone)]
pub struct ReplayFrame
{
    delta: u32,
    x: f32,
    y: f32,
    button_state: i8
}

impl ReplayFrame {
    pub fn empty() -> ReplayFrame {
        ReplayFrame {
            delta: 0,
            x: 0.0,
            y: 0.0,
            button_state: 0
        }
    }
}

pub fn compress_replay_frames(frames: Vec<ReplayFrame>) -> Vec<u8>
{
    let mut data = vec![0u8; frames.len() * 6];

    let mut index = 0;

    for frame in frames {
        let delta_button = (frame.delta as u16) | ((frame.button_state as u16) << 11);

        let x = f16::from_f32(frame.x).to_bits();
        let y = f16::from_f32(frame.y).to_bits();

        let offset = index * 6;
        data[offset+0] = (delta_button & 0xFF) as u8; data[offset+1] = ((delta_button >> 8) & 0xFF) as u8;
        data[offset+2] =            (x & 0xFF) as u8; data[offset+3] =            ((x >> 8) & 0xFF) as u8;
        data[offset+4] =            (y & 0xFF) as u8; data[offset+5] =            ((y >> 8) & 0xFF) as u8;

        index += 1;
    }

    return data;
}

pub fn decompress_replay_frames(data: Vec<u8>) -> Vec<ReplayFrame>
{
    let length = data.len() / 6;

    let mut frames = vec![ReplayFrame::empty(); length];

    for i in 0..length {
        let offset    = i * 6;

        let delta_bits = data[offset+0] as u16 | ((data[offset+1] as u16) << 8);
        let x_bits     = data[offset+2] as u16 | ((data[offset+3] as u16) << 8);
        let y_bits     = data[offset+4] as u16 | ((data[offset+5] as u16) << 8);

        frames[i] = ReplayFrame {
            delta: (delta_bits & 0x7FF) as u32,
            x: f16::from_bits(x_bits).to_f32(),
            y: f16::from_bits(y_bits).to_f32(),
            button_state: (delta_bits >> 11) as i8
        }
    }

    frames
}

#[test]
fn replay_compression() {
    let frames = vec![
        ReplayFrame {
            delta: 21,
            x: 985.52001,
            y: 570.52001,
            button_state: 1
        },
        ReplayFrame {
            delta: 256,
            x: 557.98850,
            y: 654.45578,
            button_state: 4
        },
        ReplayFrame {
            delta: 1024,
            x: 558.99999,
            y: 104.99996,
            button_state: 16
        },
    ];

    let compressed = compress_replay_frames(frames);
    let decompressed = decompress_replay_frames(compressed);

    println!("{:?}", decompressed);
}
