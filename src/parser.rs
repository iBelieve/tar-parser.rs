use std::str::from_utf8;
use nom::IResult;

#[derive(Debug,PartialEq,Eq)]
pub struct PosixHeader<'a> {
    pub name:     & 'a str,
    pub mode:     & 'a str,
    pub uid:      & 'a str,
    pub gid:      & 'a str,
    pub size:     & 'a str,
    pub mtime:    & 'a str,
    pub chksum:   & 'a str,
    pub typeflag: char,
    pub linkname: & 'a str,
    pub ustar:    Option<UStarHeader<'a>>
}

#[derive(Debug,PartialEq,Eq)]
pub struct UStarHeader<'a> {
    pub magic:    & 'a str,
    pub version:  & 'a str,
    pub uname:    & 'a str,
    pub gname:    & 'a str,
    pub devmajor: & 'a str,
    pub devminor: & 'a str,
    pub prefix:   & 'a str,
}

#[derive(Debug,PartialEq,Eq)]
pub struct TarEntry<'a> {
    pub header:   PosixHeader<'a>,
    pub contents: & 'a str
}

fn str_to_u32(s: &str, base: u32) -> u32 {
    let mut u = 0;
    let mut f = 1;

    for c in s.chars().rev() {
        u += f * ((c as u32) - ('0' as u32));
        f *= base;
    }

    u
}

pub fn octal_to_u32(o: &str) -> u32 {
    str_to_u32(o, 8)
}

pub fn decimal_to_u32(d: &str) -> u32 {
    str_to_u32(d, 10)
}

fn parse_ustar(i: &[u8]) -> IResult<&[u8], Option<UStarHeader>> {
    chain!(i,
        magic:    map_res!(take!(6),   from_utf8) ~
        version:  map_res!(take!(2),   from_utf8) ~
        uname:    map_res!(take!(32),  from_utf8) ~
        gname:    map_res!(take!(32),  from_utf8) ~
        devmajor: map_res!(take!(8),   from_utf8) ~
        devminor: map_res!(take!(8),   from_utf8) ~
        prefix:   map_res!(take!(155), from_utf8),
        ||{
            match magic {
                "ustar" => Some(UStarHeader{
                    magic:    magic,
                    version:  version,
                    uname:    uname,
                    gname:    gname,
                    devmajor: devmajor,
                    devminor: devminor,
                    prefix:   prefix
                }),
                _ => None,
            }
        }
    )
}

fn parse_header(i: &[u8]) -> IResult<&[u8], PosixHeader> {
    chain!(i,
        name:     map_res!(take!(100), from_utf8) ~
        mode:     map_res!(take!(8),   from_utf8) ~
        uid:      map_res!(take!(8),   from_utf8) ~
        gid:      map_res!(take!(8),   from_utf8) ~
        size:     map_res!(take!(12),  from_utf8) ~
        mtime:    map_res!(take!(12),  from_utf8) ~
        chksum:   map_res!(take!(8),   from_utf8) ~
        typeflag: take!(1)                        ~
        linkname: map_res!(take!(100), from_utf8) ~
        ustar:    parse_ustar                     ~
        take!(12), /* padding to 512 */
        ||{
            PosixHeader {
                name:     name,
                mode:     mode,
                uid:      uid,
                gid:      gid,
                size:     size,
                mtime:    mtime,
                chksum:   chksum,
                typeflag: typeflag[0] as char,
                linkname: linkname,
                ustar:    ustar
            }
        }
    )
}

fn parse_entry(i: &[u8]) -> IResult<&[u8], TarEntry> {
    chain!(i,
        header: parse_header,
        /* TODO: contents */
        ||{
            TarEntry {
                header: header,
                contents: ""
            }
        }
    )
}

pub fn parse_tar(i: &[u8]) -> IResult<&[u8], Vec<TarEntry>> {
    many0!(i, parse_entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn octal_to_u32_test() {
        match octal_to_u32("756") {
            494 => {},
            o => panic!("octal_to_u32 failed, expected 494 but got {}", o)
        }
        match octal_to_u32("") {
            0 => {},
            o => panic!("octal_to_u32 failed, expected 0 but got {}", o)
        }
    }

    #[test]
    fn decimal_to_u32_test() {
        match decimal_to_u32("756") {
            756 => {},
            d => panic!("decimal_to_u32 failed, expected 756 but got {}", d)
        }
        match decimal_to_u32("") {
            0 => {},
            d => panic!("decimal_to_u32 failed, expected 0 but got {}", d)
        }
    }
}