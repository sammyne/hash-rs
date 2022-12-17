use std::ptr;

lazy_static::lazy_static! {
  static ref HASH_KEY: [usize;4] = {
    let mut b=[0u8;32];
    getrandom::getrandom(&mut b).expect("[HASH_KEY] rand data");

    let mut out = [0usize;4];
    for (i,v) in b.chunks_exact(8).enumerate(){
      let w:[u8;8] = v.try_into().expect("[HASH_KEY] &[u8] as [u8; 8]");
      out[i]= usize::from_be_bytes(w);
    }

    out
  };
}

const M1: usize = 0xa0761d6478bd642f;
const M2: usize = 0xe7037ed1a0b428db;
const M3: usize = 0x8ebc6af09c88c6e3;
const M4: usize = 0x589965cc75374cc3;
const M5: usize = 0x1d8e4e27c47d124f;

/// ref: https://github.com/golang/go/blob/go1.19.4/src/runtime/hash64.go#L25
pub unsafe fn sum<T>(addr: &T, seed: usize, s: usize) -> usize
where
    T: ?Sized,
{
    let mut p = addr as *const T as *const u8;

    //let (mut a, mut b) = (0usize, 0usize);

    let mut seed = seed ^ (HASH_KEY[0] ^ M1);
    let (a, b) = match s {
        0 => return seed,
        1..=3 => {
            let mut a = *p as usize;
            a |= ((*p.offset((s >> 1) as isize)) as usize) << 8;
            a |= (*(p.offset((s - 1) as isize)) as usize) << 16;
            (a, 0)
        }
        4 => {
            let a = r4(p);
            (a, a)
        }
        5..=7 => {
            let a = r4(p);
            let b = r4(p.offset((s - 4) as isize));
            (a, b)
        }
        8 => {
            let a = r8(p);
            (a, a)
        }
        9..=16 => {
            let a = r8(p);
            let b = r8(p.offset((s - 4) as isize));
            (a, b)
        }
        _ => {
            let mut l = s as isize;
            if l > 48 {
                let (mut s1, mut s2) = (seed, seed);
                while l > 48 {
                    seed = mix(r8(p) ^ M2, r8(p.offset(8)) ^ seed);
                    s1 = mix(r8(p.offset(16)) ^ M3, r8(p.offset(24)) ^ s1);
                    s2 = mix(r8(p.offset(32)) ^ M4, r8(p.offset(40)) ^ s2);
                    p = p.offset(48);
                    l -= 48;
                }
                seed ^= s1 ^ s2;
            }
            while l > 16 {
                seed = mix(r8(p) ^ M2, r8(p.offset(8)) ^ seed);
                p = p.offset(16);
                l -= 16;
            }
            let a = r8(p.offset(l - 16));
            let b = r8(p.offset(l - 8));
            (a, b)
        }
    };

    mix(M5 ^ s, mix(a ^ M2, b ^ seed))
}

fn mix(a: usize, b: usize) -> usize {
    let p = (a as u128).wrapping_mul(b as u128);
    let (hi, lo) = ((p >> 64) as u64, p as u64);
    (hi ^ lo) as usize
}

unsafe fn r4(p: *const u8) -> usize {
    let p = p as *const u32;
    ptr::read_unaligned(p) as usize
}

unsafe fn r8(p: *const u8) -> usize {
    let p = p as *const u64;
    ptr::read_unaligned(p) as usize
}
