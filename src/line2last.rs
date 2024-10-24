use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;

pub const LINE_SEPARATOR_DEFAULT: u8 = b'\n';
pub const LAST_BYTES_DEFAULT: usize = 8;

fn len2start_index(len: usize, num_last_bytes: usize) -> usize {
    len.saturating_sub(num_last_bytes)
}

pub fn line2last(line: &[u8], cnt: usize) -> &[u8] {
    let start_ix: usize = len2start_index(line.len(), cnt);
    &line[start_ix..]
}

#[cfg(test)]
mod test_line2last {
    mod len2start_index {
        use crate::line2last;

        #[test]
        fn empty() {
            let got: usize = line2last::len2start_index(0, 8);
            assert_eq!(got, 0);
        }

        #[test]
        fn zero() {
            let got: usize = line2last::len2start_index(8, 8);
            assert_eq!(got, 0);
        }

        #[test]
        fn ix() {
            let got: usize = line2last::len2start_index(9, 8);
            assert_eq!(got, 1);
        }
    }

    mod line2last {
        use crate::line2last;

        #[test]
        fn empty() {
            let got: &[u8] = line2last::line2last(b"", 8);
            assert_eq!(got, b"");
        }

        #[test]
        fn equal() {
            let got: &[u8] = line2last::line2last(b"20241024", 8);
            assert_eq!(got, b"20241024");
        }

        #[test]
        fn longer() {
            let got: &[u8] = line2last::line2last(b"2024-10-24T13:36:13.01234567", 8);
            assert_eq!(got, b"01234567");
        }
    }
}

pub fn lines2last2output<I, L, O>(
    lines: I,
    output: &mut O,
    last_cnt: usize,
    line2last: &L,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Vec<u8>>,
    L: Fn(&[u8], usize) -> &[u8],
    O: FnMut(&[u8]) -> Result<(), io::Error>,
{
    for line in lines {
        let last_bytes: &[u8] = line2last(&line, last_cnt);
        output(last_bytes)?;
    }
    Ok(())
}

pub fn reader2writer<R, L, W>(
    rdr: R,
    wtr: &mut W,
    last_cnt: usize,
    line2last: &L,
    line_sep: u8,
) -> Result<(), io::Error>
where
    R: Read,
    L: Fn(&[u8], usize) -> &[u8],
    W: Write,
{
    let br = BufReader::new(rdr);
    let lines = br.split(line_sep);
    let noerr = lines.map_while(Result::ok);
    {
        let mut bw = BufWriter::new(wtr.by_ref());
        lines2last2output(
            noerr,
            &mut |last_bytes: &[u8]| {
                bw.write_all(last_bytes)?;
                bw.write_all(&[line_sep])?;
                Ok(())
            },
            last_cnt,
            line2last,
        )?;
        bw.flush()?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn stdin2stdout<L>(last_cnt: usize, line2last: &L, line_sep: u8) -> Result<(), io::Error>
where
    L: Fn(&[u8], usize) -> &[u8],
{
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let mut ol = o.lock();
    reader2writer(il, &mut ol, last_cnt, line2last, line_sep)?;
    ol.flush()?;
    Ok(())
}

pub fn stdin2stdout_default(last_cnt: usize) -> Result<(), io::Error> {
    stdin2stdout(last_cnt, &line2last, LINE_SEPARATOR_DEFAULT)
}
