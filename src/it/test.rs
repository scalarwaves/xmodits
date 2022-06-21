use super::it::*;

#[test]
fn test1() {
    let a = ItFile::load("samples/NYCStreets_Music.it").unwrap();
    // a.export("./test/longhorn_test_5.wav", 9).unwrap();
    // for i in 0..a.sample_number {
    //     let _ = a.export(format!("./test/{}.wav", i), i as usize);
    // }

    for i in 0..10 {
        println!("{}", i);
        println!(
            "sample flags: {:08b}\n",
            &a.samples_meta[i].smp_flag,
        );
    }

    // for i in 0..89 {
    //     println!("{}", i);
    //     println!(
    //         "sample length: {}\nsample pointer {:04X}\nsample speed: {}\nsample flags: {:08b}\n\n",
    //         &a.samples_meta[i].length,
    //         &a.samples_meta[i].sample_pointer,
    //         &a.samples_meta[i].sample_rate,
    //         &a.samples_meta[i].flags,
    //     );
    // }
}
#[test]
fn test_flag_set() {
    const MASK_SMP_BITS: u8 = 0b0000_0010;
    let test_func = |b:u8| {8 * (((b & MASK_SMP_BITS) >> 1)  + 1)};
    let f1_8    = 0b010100_0_1;    // should be 8 
    let f2_16   = 0b000000_1_1;   // should be 16

    assert_eq!(test_func(f1_8), 8);
    assert_eq!(test_func(f2_16), 16);
}

#[test]
fn test_flag_set_2() {
    const MASK_SMP_BITS: u8 = 0b0000_1000;
    let test_func = |b:u8| {((b & MASK_SMP_BITS) >> 3) == 1};
    let f1_false    = 0b0101_0_001;    // should be false
    let f2_true     = 0b0000_1_011;   // should be true

    assert_eq!(test_func(f1_false), false);
    assert_eq!(test_func(f2_true), true);
}