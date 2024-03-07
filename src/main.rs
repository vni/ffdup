const BLOCK_SIZE: usize = 4096;

struct Entry {
    hash: blake3::Hash,
    path: String,
}

struct Flags {
    show_only_duplicates: bool,
    show_all: bool,
    show_hashes: bool,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}    {}", self.hash, self.path)
    }
}

fn calculate_hash(path: String) -> Result<Entry, String> {
    use std::io::Read;

    let mut file = std::fs::File::open(path.clone()).unwrap();
    let mut buf = [0u8; BLOCK_SIZE];

    let mut blake3_hasher = blake3::Hasher::new();

    loop {
        let r = file.read(&mut buf);
        if r.is_err() {
            return Err(format!("Can't read file '{}', because of '{:?}'", path, r));
        }

        let n = r.unwrap();
        if n == 0 {
            break;
        }

        blake3_hasher.update(&buf[..n]);
    }
    let hash = blake3_hasher.finalize();

    Ok(Entry { hash, path })
}

fn visit_dir(total: &mut Vec<Entry>, path: String) -> usize {
    use std::io::Write;

    let mut count: usize = 0;
    for dirent in std::fs::read_dir(path).unwrap() {
        let dirent = dirent.unwrap();
        let path = dirent.path();
        if path.is_dir() {
            count += visit_dir(total, path.to_string_lossy().to_string());
        } else {
            let entry = calculate_hash(path.to_string_lossy().to_string()).unwrap();
            count += 1;
            print!(".");
            std::io::stdout().flush().unwrap();
            total.push(entry);
        }
    }

    count
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("At least one filename should be passed: fidup <file1> <file2> <file3> ...");
        std::process::exit(1);
    }

    let flags = Flags {
        show_only_duplicates: false,
        show_all: false,
        show_hashes: true,
    };

    let mut file_count: usize = 0;
    let mut total = Vec::new();
    for path in args.into_iter().skip(1) {
        file_count += visit_dir(&mut total, path);
    }
    total.sort_by(|a, b| a.hash.as_bytes().partial_cmp(b.hash.as_bytes()).unwrap());

    let mut prev = Entry {
        hash: blake3::Hash::from_bytes([0u8; 32]),
        path: String::new(),
    };
    let mut n_duplicates = 0;
    for cur in total {
        if prev.hash == cur.hash {
            n_duplicates += 1;
            println!();
            if flags.show_hashes {
                println!("{}", prev.hash);
            }
            println!("{}", prev.path);
            println!("{}", cur.path);
            println!();
        } else if flags.show_all {
            println!("{} {}", cur.hash, cur.path);
        }
        prev = cur;
    }
    println!("files scanned: {}", file_count);
    println!("# of duplicates: {}", n_duplicates);
}
