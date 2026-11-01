use std::collections::HashMap;
use std::io::Write;

#[derive(Default)]
struct Stat {
    lat: Vec<u32>,
    errors: u64,
}

#[inline]
fn num(b: &[u8]) -> u32 {
    let mut v = 0u32;
    for &c in b {
        v = v * 10 + (c - b'0') as u32;
    }
    v
}

#[inline]
fn hour_key(ts: &[u8]) -> u64 {
    (((num(&ts[0..4]) as u64 * 100 + num(&ts[5..7]) as u64) * 100 + num(&ts[8..10]) as u64) * 100)
        + num(&ts[11..13]) as u64
}

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/data/requests.log".into());
    let data = std::fs::read(&path).expect("read input");

    let mut map: HashMap<(u64, &[u8]), Stat> = HashMap::with_capacity(100_000);

    for line in data.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let mut f = line.split(|&b| b == b' ');
        let ts = f.next().unwrap();
        let _method = f.next().unwrap();
        let ep = f.next().unwrap();
        let status = num(f.next().unwrap());
        let lat = num(f.next().unwrap());

        let s = map.entry((hour_key(ts), ep)).or_default();
        s.lat.push(lat);
        if (500..=599).contains(&status) {
            s.errors += 1;
        }
    }

    let mut groups: Vec<_> = map.into_iter().collect();
    groups.sort_unstable_by(|((hour_a, endpoint_a), _), ((hour_b, endpoint_b), _)| {
        hour_a.cmp(hour_b).then_with(|| endpoint_a.cmp(endpoint_b))
    });

    let out = std::env::var("OUT").unwrap_or_else(|_| "/out/summary.txt".into());
    let mut w = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    for ((hour, ep), mut s) in groups {
        s.lat.sort_unstable();
        let n = s.lat.len();
        let p = |percent: usize| {
            let rank = (percent * n + 99) / 100;
            s.lat[rank - 1]
        };
        let year = hour / 1_000_000;
        let month = hour / 10_000 % 100;
        let day = hour / 100 % 100;
        let hour_of_day = hour % 100;
        let ppm = s.errors * 1_000_000 / n as u64;
        writeln!(
            w,
            "{:04}-{:02}-{:02}T{:02}:00:00Z {} {} {} {} {} {} {}",
            year,
            month,
            day,
            hour_of_day,
            std::str::from_utf8(ep).unwrap(),
            n,
            p(50),
            p(95),
            p(99),
            s.errors,
            ppm
        )
        .unwrap();
    }
}
