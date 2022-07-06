use byteorder::{ByteOrder, BE, LE};
use crate::{word, dword};

const RIFF: u32 = 0x5249_4646; // RIFF
const WAVE: u32 = 0x5741_5645; // WAVE
const FMT_: u32 = 0x666D_7420; // "riff "
const DATA: u32 = 0x6461_7461; // data
const HEADER_SIZE: u8 = 44;

pub fn build_header(
    smp_rate: u32,  // sample rate
    smp_bits: u8,   // bits per sample
    pcm_len: u32,   // length of byte array
    stereo: bool,   // is pcm stereo
) -> [u8; HEADER_SIZE as usize]
{
    let mut header:[u8; HEADER_SIZE as usize] = [0u8; HEADER_SIZE as usize];     
    let wav_scs:            u32 = 16;                       // sec chunk size
    let wav_type:           u16 = 1;                        // 1 = pcm
    let monostereo:         u16 = stereo as u16 + 1;         // 0x01 = mono, 0x02 = stereo
    let sample_frequency:   u32 = smp_rate;
    let bytes_sec:          u32 = smp_rate * monostereo as u32;   // sample_rate * channels (DOUBLE CHECK)
    let block_align:        u16 = 0x01;                     // can be anything really
    let bits_sample:        u16 = smp_bits as u16;
    let file_size:          u32 = HEADER_SIZE as u32 + (pcm_len * (bits_sample / 8) as u32) - 8;
    let size_of_chunk:      u32 = pcm_len * (bits_sample / 8) as u32;

    BE::write_u32(&mut header[dword!(0x0000)], RIFF);
    LE::write_u32(&mut header[dword!(0x0004)], file_size);
    BE::write_u32(&mut header[dword!(0x0008)], WAVE);
    BE::write_u32(&mut header[dword!(0x000C)], FMT_);
    LE::write_u32(&mut header[dword!(0x0010)], wav_scs);
    LE::write_u16(&mut header[word!(0x0014)], wav_type);
    LE::write_u16(&mut header[word!(0x0016)], monostereo);
    LE::write_u32(&mut header[dword!(0x0018)], sample_frequency);
    LE::write_u32(&mut header[dword!(0x001C)], bytes_sec);
    LE::write_u16(&mut header[word!(0x0020)], block_align);
    LE::write_u16(&mut header[word!(0x0022)], bits_sample);
    BE::write_u32(&mut header[dword!(0x0024)], DATA);
    LE::write_u32(&mut header[dword!(0x0028)], size_of_chunk);
    
    header
}