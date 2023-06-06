/* Copyright (c) Fortanix, Inc.
 *
 * Licensed under the GNU General Public License, version 2 <LICENSE-GPL or
 * https://www.gnu.org/licenses/gpl-2.0.html> or the Apache License, Version
 * 2.0 <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>, at your
 * option. This file may not be copied, modified, or distributed except
 * according to those terms. */

use mbedtls::bignum::{Mpi, Sign};

#[cfg(feature = "std")]
#[test]
fn bignum_from_str() {
    use std::str::FromStr;

    let p256_16 =
        Mpi::from_str("0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff")
            .unwrap();
    let p256_10 = Mpi::from_str(
        "115792089210356248762697446949407573530086143415290314195533631308867097853951",
    )
    .unwrap();

    assert!(p256_16.eq(&p256_10));

    assert_eq!(
        format!("{}", p256_10),
        "115792089210356248762697446949407573530086143415290314195533631308867097853951"
    );
    assert_eq!(
        format!("{:X}", p256_10),
        "FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF"
    );
    assert_eq!(
        format!("{:o}", p256_10),
        "17777777777400000000010000000000000000000000000000000077777777777777777777777777777777"
    );
    assert_eq!(format!("{:b}", p256_10), "1111111111111111111111111111111100000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
}

#[test]
fn bignum() {
    let six = Mpi::new(6).unwrap();

    assert_eq!(six.byte_length().unwrap(), 1);
    assert_eq!(six.bit_length().unwrap(), 3);

    let six_bytes = six.to_binary().unwrap();
    assert_eq!(six_bytes.len(), 1);
    assert_eq!(six_bytes[0], 6);

    let five = Mpi::new(5).unwrap();
    assert_eq!(six.cmp(&five), ::std::cmp::Ordering::Greater);
    assert_eq!(five.cmp(&five), ::std::cmp::Ordering::Equal);
    assert_eq!(five.cmp(&six), ::std::cmp::Ordering::Less);

    let bigger = Mpi::new(0x2a2f5dce).unwrap();

    assert_eq!(bigger.byte_length().unwrap(), 4);
    assert_eq!(bigger.bit_length().unwrap(), 30);

    let b_bytes = bigger.to_binary().unwrap();
    assert_eq!(b_bytes.len(), 4);
    assert_eq!(b_bytes[0], 0x2a);
    assert_eq!(b_bytes[1], 0x2f);
    assert_eq!(b_bytes[2], 0x5d);
    assert_eq!(b_bytes[3], 0xce);

    assert!(bigger.eq(&Mpi::from_binary(&b_bytes).unwrap()));
}

#[test]
fn bignum_shifts() {
    let x = Mpi::new(3).unwrap();

    let y = (&x << 30).unwrap();

    assert_eq!(format!("{}", y), "3221225472");

    let y = (&y >> 30).unwrap();

    assert_eq!(format!("{}", y), "3");

    let y = (&y >> 2).unwrap();

    assert_eq!(format!("{}", y), "0");

    let mut z = Mpi::new(1).unwrap();

    z <<= 5;
    assert_eq!(format!("{}", z), "32");
    z <<= 15;
    assert_eq!(format!("{}", z), "1048576");

    z >>= 10;
    assert_eq!(format!("{}", z), "1024");
}

#[test]
fn bignum_op_assign() {
    let mut x = Mpi::new(4).unwrap();

    x += 9;

    assert_eq!(format!("{}", x), "13");

    x += Mpi::new(13).unwrap();

    assert_eq!(format!("{}", x), "26");

    let y = Mpi::new(10).unwrap();
    x += &y;

    assert_eq!(format!("{}", x), "36");

    x -= 3;
    assert_eq!(format!("{}", x), "33");

    x -= Mpi::new(5).unwrap();
    assert_eq!(format!("{}", x), "28");

    x -= &y;
    assert_eq!(format!("{}", x), "18");

    x *= &y;
    assert_eq!(format!("{}", x), "180");

    x *= 2;
    assert_eq!(format!("{}", x), "360");

    assert_eq!(x.sign(), Sign::Positive);
    x *= Mpi::new(-2).unwrap();
    assert_eq!(format!("{}", x), "-720");
    assert_eq!(x.sign(), Sign::Negative);

    x /= Mpi::new(-3).unwrap();
    assert_eq!(format!("{}", x), "240");

    x /= 2;
    assert_eq!(format!("{}", x), "120");

    x /= &y;
    assert_eq!(format!("{}", x), "12");

    x %= 100;
    assert_eq!(format!("{}", x), "12");

    x %= Mpi::new(5).unwrap();
    assert_eq!(format!("{}", x), "2");

    assert_eq!(format!("{}", y), "10"); // verify y not moved
}

#[cfg(feature = "std")]
#[test]
fn test_jacobi_fn() {
    use std::str::FromStr;

    fn jacobi_symbol_test(a: &str, n: &str, expected: i32) {
        let a = Mpi::from_str(a).unwrap();
        let n = Mpi::from_str(n).unwrap();
        let j = a.jacobi(&n).unwrap();
        //println!("a={} n={} J={}", a, n, j);
        assert_eq!(j, expected);
    }

    // Tests generated by Sagemath
    jacobi_symbol_test("5", "9", 1);
    jacobi_symbol_test(
        "80530568503105393620776136885268819039",
        "136759011081214619901277936869624817013",
        -1,
    );
    jacobi_symbol_test("541641436", "50733077", -1);
    jacobi_symbol_test("541641437", "50733077", 1);
    jacobi_symbol_test("50733077", "50733077", 0);
    jacobi_symbol_test("126192963", "2869415899", 1);
    jacobi_symbol_test("126192964", "2869415899", -1);

    jacobi_symbol_test(
        "290122183148875935619099270547",
        "392382503032982745991600930111",
        -1,
    );
    jacobi_symbol_test(
        "652189681324592774835681787902",
        "851019412553174450003757422011",
        1,
    );
    jacobi_symbol_test(
        "68607521964935451958858272376",
        "89491088927603607083107403767",
        1,
    );
    jacobi_symbol_test(
        "218068701715357900365812660263",
        "238095134266847041021320150827",
        -1,
    );
    jacobi_symbol_test(
        "9847597030024907406584779047",
        "20414312383664964481261270711",
        1,
    );
    jacobi_symbol_test(
        "38938513347318987388516082474",
        "49516772312071161029219932219",
        1,
    );
    jacobi_symbol_test(
        "300820947915083731970108494721",
        "657305681340895250386089542863",
        -1,
    );
    jacobi_symbol_test(
        "12565726709694140412667952162",
        "31771076028760826448147679003",
        -1,
    );
    jacobi_symbol_test(
        "344945231515347227453035588988",
        "828252022515408040124517036011",
        1,
    );
    jacobi_symbol_test(
        "93331799786934264132380785163",
        "313205417670262818093976413871",
        -1,
    );
}

#[cfg(feature = "std")]
#[test]
fn test_mod_sqrt_fn() {
    use std::str::FromStr;

    fn mod_sqrt_test(a: &str, n: &str, expected: &str) {
        let a = Mpi::from_str(a).unwrap();
        let n = Mpi::from_str(n).unwrap();
        let expected = Mpi::from_str(expected).unwrap();
        let mut computed = a.mod_sqrt(&n).unwrap();

        /*
        If x = (a*a) mod p then also x = (-a*-a) mod p, ie
        if a square root exists then there are two square roots related by
        x and p-x. The mod sqrt might return either of these options
        */
        if &computed != &expected {
            computed = (&n - &computed).unwrap();
        }
        assert_eq!(computed, expected);
    }

    // Tests generated by Sagemath
    mod_sqrt_test("2", "7", "4");
    mod_sqrt_test("5", "469289024411159", "234325000312516");
    mod_sqrt_test(
        "458050473005020050313790240477",
        "905858848829014223214249213947",
        "126474086260479574845714194337",
    );

    mod_sqrt_test("4", "13", "2");
    mod_sqrt_test("2", "113", "62");

    mod_sqrt_test(
        "14432894130216089699367965001582109139186342668614313620824414613061488655",
        "145226202540352375281647974706811878790868025723961296389762379073201613561",
        "64346440714386899555372506097606752274599811989145306413544609746921648646",
    );

    mod_sqrt_test(
        "2",
        "145226202540352375281647974706811878790868025723961296389762379073201613561",
        "29863506841820532608636271306847583140720915984413766535227954746838873278",
    );

    mod_sqrt_test(
        "2",
        "0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF",
        "0x507442007322AA895340CBA4ABC2D730BFD0B16C2C79A46815F8780D2C55A2DD",
    );

    mod_sqrt_test(
        "0x5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B",
        "0xFFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF",
        "0x99B7A386F1D07C29DBCC42A27B5F9449ABE3D50DE25178E8D7407A95E8B06C0B",
    );

    // Primes where 2^s divides p-1 for s >= 3 which caught a bug
    mod_sqrt_test("2", "17", "6");
    mod_sqrt_test("2", "97", "14");
    mod_sqrt_test("2", "193", "52");
    mod_sqrt_test("2", "257", "60");
    mod_sqrt_test("2", "65537", "4080");

    mod_sqrt_test("2", "0x1200000001", "17207801277");

    mod_sqrt_test(
        "2",
        "0x660000000000000000000000000000000000000000000000000000000000000001",
        "0xce495874f10d32d28105400c73f73aafc7cbbae7cd1dfa1525f2701b3573d78c0",
    );
}

#[test]
fn bignum_cmp() {
    let big = Mpi::new(2147483647).unwrap();
    let small = Mpi::new(2).unwrap();

    assert!(big > small);
    assert!(small < big);
    assert!(big >= small);
    assert!(small <= big);
    assert!(small >= small);
    assert!(big <= big);
    assert!(small == small);
    assert!(small != big);
}

#[test]
fn bigint_ops() {
    let x = Mpi::new(100).unwrap();
    let y = Mpi::new(20900).unwrap();

    assert_eq!(x.as_u32().unwrap(), 100);

    let z = (&x + &y).unwrap();
    assert_eq!(z.as_u32().unwrap(), 21000);

    let z = (&z * &y).unwrap();
    assert_eq!(z, Mpi::new(438900000).unwrap());

    let z = (&z - &x).unwrap();
    assert_eq!(z, Mpi::new(0x1A2914BC).unwrap());

    let r = (&z % 127).unwrap();
    assert_eq!(r.as_u32().unwrap(), 92);

    let r = (&z % &Mpi::new(127).unwrap()).unwrap();
    assert_eq!(r.as_u32().unwrap(), 92);

    let q = (&z / 53).unwrap();
    assert_eq!(q.as_u32().unwrap(), 8281130);

    let q = (&z / &Mpi::new(53).unwrap()).unwrap();
    assert_eq!(q.as_u32().unwrap(), 8281130);

    let nan = &z / 0;
    assert!(nan.is_err());
}

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn base58_encode(bits: &[u8]) -> mbedtls::Result<String> {
    let zero = Mpi::new(0)?;
    assert_eq!(zero.sign(), Sign::Zero);
    let mut n = Mpi::from_binary(bits)?;
    let radix: i64 = 58;

    let mut s = Vec::new();

    while n > zero {
        let (q, r) = n.divrem_int(radix)?;
        n = q;
        s.push(BASE58_ALPHABET[r.as_u32()? as usize]);
    }

    s.reverse();
    Ok(String::from_utf8(s).unwrap())
}

fn base58_decode(b58: &str) -> mbedtls::Result<Vec<u8>> {
    let radix: i64 = 58;

    let mut n = Mpi::new(0)?;

    fn base58_val(b: u8) -> mbedtls::Result<usize> {
        for (i, c) in BASE58_ALPHABET.iter().enumerate() {
            if *c == b {
                return Ok(i);
            }
        }
        Err(mbedtls::error::codes::Base64InvalidCharacter.into())
    }

    for c in b58.bytes() {
        let v = base58_val(c)? as i64;
        n = (&n * radix)?;
        n = (&n + v)?;
    }

    n.to_binary()
}

#[test]
fn test_base58_encode() {
    fn test_base58_rt(input: &[u8], expected: &str) {
        assert_eq!(base58_encode(input).unwrap(), expected);
        assert_eq!(base58_decode(expected).unwrap(), input);
    }

    test_base58_rt(b"", "");
    test_base58_rt(&[32], "Z");
    test_base58_rt(&[45], "n");
    test_base58_rt(&[48], "q");
    test_base58_rt(&[49], "r");
    test_base58_rt(&[57], "z");
    test_base58_rt(&[45, 49], "4SU");
    test_base58_rt(&[49, 49], "4k8");
    test_base58_rt(b"abc", "ZiCa");
    test_base58_rt(b"1234598760", "3mJr7AoUXx2Wqd");
    test_base58_rt(
        b"abcdefghijklmnopqrstuvwxyz",
        "3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f",
    );
}

#[cfg(feature = "rdrand")]
#[test]
fn is_probably_prime() {
    fn test_is_prime(input: Mpi, prime_expected: bool) {
        use mbedtls::rng::{CtrDrbg, Rdseed};
        use mbedtls::error::codes::MpiNotAcceptable;
        let mut rng = CtrDrbg::new(Rdseed.into(), None).unwrap();
        let rounds = 15; // accuracy 1/2**30

        let result = input.is_probably_prime(rounds, &mut rng);
        if prime_expected {
            assert!(result.is_ok());
        } else {
            assert!(result == Err(MpiNotAcceptable.into()), "expected not prime");
        }
    }
    test_is_prime(Mpi::new(-4).unwrap(), false);
    test_is_prime(Mpi::new(-3).unwrap(), true);
    test_is_prime(Mpi::new(-2).unwrap(), true);
    test_is_prime(Mpi::new(-1).unwrap(), false);
    test_is_prime(Mpi::new(0).unwrap(), false);
    test_is_prime(Mpi::new(1).unwrap(), false);
    test_is_prime(Mpi::new(2).unwrap(), true);
    test_is_prime(Mpi::new(3).unwrap(), true);
    test_is_prime(Mpi::new(4).unwrap(), false);
    test_is_prime(Mpi::new(9973).unwrap(), true);   // Last prime before 10000
    test_is_prime(Mpi::new(10007).unwrap(), true);  // First prime after 10000
    test_is_prime(Mpi::new(10009).unwrap(), true);
    test_is_prime(Mpi::new(22801763489).unwrap(), true);  // 1000000000th prime
    test_is_prime(Mpi::new(22801763491).unwrap(), false);

    // RSA-250, the largest RSA challenge solved (829 bits)
    let rsa_250_n = Mpi::from_binary(&[
        0x13, 0x21, 0xD2, 0xFD, 0xDD, 0xE8, 0xBD, 0x9D, 0xFF, 0x37, 0x9A, 0xFF,
        0x03, 0x0D, 0xE2, 0x05, 0xB8, 0x46, 0xEB, 0x5C, 0xEC, 0xC4, 0x0F, 0xA8,
        0xAA, 0x9C, 0x2A, 0x85, 0xCE, 0x3E, 0x99, 0x21, 0x93, 0xE8, 0x73, 0xB2,
        0xBC, 0x66, 0x7D, 0xAB, 0xE2, 0xAC, 0x3E, 0xE9, 0xDD, 0x23, 0xB3, 0xA9,
        0xED, 0x9E, 0xC0, 0xC3, 0xC7, 0x44, 0x56, 0x63, 0xF5, 0x45, 0x54, 0x69,
        0xB7, 0x27, 0xDD, 0x6F, 0xBC, 0x03, 0xB1, 0xBF, 0x95, 0xD0, 0x3A, 0x13,
        0xC0, 0x36, 0x86, 0x45, 0x76, 0x76, 0x30, 0xC7, 0xEA, 0xBF, 0x5E, 0x7A,
        0xB5, 0xFA, 0x27, 0xB9, 0x4A, 0xDE, 0x7E, 0x1E, 0x23, 0xBC, 0xC6, 0x5D,
        0x2A, 0x7D, 0xED, 0x1C, 0x5B, 0x36, 0x4B, 0x51]
    ).unwrap();
    let rsa_250_p = Mpi::from_binary(&[
        0x61, 0x04, 0xFA, 0xF8, 0x1F, 0x41, 0xFD, 0xD7, 0x61, 0x6B, 0x43, 0x78,
        0xF6, 0xBD, 0x99, 0x12, 0x92, 0xCB, 0x2F, 0x21, 0xC1, 0x0D, 0x06, 0xC5,
        0xE8, 0xE5, 0x71, 0xA5, 0xE9, 0x62, 0xB7, 0xE8, 0x2D, 0xFD, 0x9F, 0xE7,
        0x12, 0x0F, 0x6D, 0x03, 0xA8, 0x6C, 0xC6, 0xBB, 0xC7, 0xDD, 0x3A, 0x62,
        0x80, 0x83, 0x9E, 0xF7]
    ).unwrap();
    let rsa_250_q = Mpi::from_binary(&[
        0x32, 0x7B, 0x9F, 0xDA, 0x4B, 0x21, 0x1E, 0x3B, 0xFD, 0xB5, 0x4F, 0x68,
        0x0E, 0x5C, 0x04, 0x52, 0x8A, 0xAA, 0x20, 0x42, 0x8A, 0xE0, 0x08, 0xFA,
        0xF4, 0x8D, 0xF6, 0xC9, 0x13, 0xF5, 0x74, 0x7D, 0x86, 0x08, 0xA5, 0xA4,
        0x8E, 0x2B, 0xFE, 0x41, 0xFA, 0xE7, 0xA0, 0x46, 0x83, 0xF2, 0x30, 0x58,
        0x52, 0xCD, 0xAD, 0xF7]
    ).unwrap();
    test_is_prime(rsa_250_p.clone(), true);
    test_is_prime(rsa_250_q.clone(), true);
    test_is_prime(rsa_250_n.clone(), false);
}
