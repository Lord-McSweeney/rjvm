use crate::reader::{FileData, ReadError, Reader};

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use miniz_oxide::inflate::decompress_to_vec;

// End-of-central-directory magic
const EOCD_MAGIC: u32 = 0x06054b50;
// Central directory file header magic
const CDFH_MAGIC: u32 = 0x02014b50;
// Local file header magic
const LFH_MAGIC: u32 = 0x04034b50;

pub struct ZipFile {
    data: Vec<u8>,

    records: HashMap<Box<[u8]>, FileRecord>,
}

impl ZipFile {
    pub fn new(data: Vec<u8>) -> Result<Self, ReadError> {
        let mut reader = FileData::new(&data);

        let (records_count, first_record_offset) = read_eocd(&mut reader)?;

        let records = read_records(&mut reader, records_count, first_record_offset)?;

        Ok(Self { data, records })
    }

    pub fn has_file(&self, file_name: &String) -> bool {
        let record_name = Box::from(file_name.as_bytes());
        self.records.contains_key(&record_name)
    }

    pub fn read_file(&self, file_name: &String) -> Result<Vec<u8>, ()> {
        let record_name = Box::from(file_name.as_bytes());
        if let Some(record) = self.records.get(&record_name) {
            // We had the record, so seek to the data
            let mut reader = FileData::new(&self.data);
            reader.seek(record.header_position).map_err(|_| ())?;
            if reader.read_u32_le().map_err(|_| ())? != LFH_MAGIC {
                return Err(());
            }

            let seek_pos = record.header_position + record.local_file_header_size;
            reader.seek(seek_pos).map_err(|_| ())?;

            let raw_bytes = reader.read_bytes(record.compressed_size).map_err(|_| ())?;

            match record.compression_method {
                0 => {
                    // no compression
                    Ok(raw_bytes)
                }
                8 => {
                    // Deflate
                    decompress_to_vec(&raw_bytes).map_err(|_| ())
                }
                unknown => {
                    panic!("Unimplemented compression method for file: {}", unknown)
                }
            }
        } else {
            Err(())
        }
    }
}

struct FileRecord {
    // Which compression method this record uses
    compression_method: usize,

    // The compressed size of the file data
    compressed_size: usize,

    // The position of the file's local file header from the start of the file
    header_position: usize,

    // The size of the local file header, precomputed so that we can skip over
    // it quickly when we're reading the file
    local_file_header_size: usize,
}

/// Read an end-of-central-directory record and return relevant data as a tuple
/// of `(records_count, first_record_offset)`
fn read_eocd<'a>(reader: &mut FileData<'a>) -> Result<(usize, usize), ReadError> {
    // assume the "comment" is of 0 length :P
    // That means we can just seek to `(end - 0x16)`
    reader.seek(reader.len() - 0x16)?;

    // We should be at the start of the EOCD now
    let magic = reader.read_u32_le()?;
    if magic != EOCD_MAGIC {
        return Err(ReadError::InvalidMagic);
    }

    // Obsolete field
    reader.read_u16_le()?;

    // Obsolete field
    reader.read_u16_le()?;

    // Obsolete field
    reader.read_u16_le()?;

    // Total number of central directory records
    let records_count = reader.read_u16_le()?;

    // Total size of all central directory records
    reader.read_u32_le()?;

    // Offset of first central directory record
    let first_record_offset = reader.read_u32_le()?;

    Ok((records_count as usize, first_record_offset as usize))
}

/// Read all the central directory records in a zip file, provided their count
/// and the offset of the first one
fn read_records<'a>(
    reader: &mut FileData<'a>,
    records_count: usize,
    first_record_offset: usize,
) -> Result<HashMap<Box<[u8]>, FileRecord>, ReadError> {
    reader.seek(first_record_offset)?;

    let mut map = HashMap::with_capacity(records_count);
    for _ in 0..records_count {
        let magic = reader.read_u32_le()?;
        if magic != CDFH_MAGIC {
            return Err(ReadError::InvalidMagic);
        }

        // Version made by
        reader.read_u16_le()?;

        // Version needed to extract
        reader.read_u16_le()?;

        // Some bitflags, not really important for us
        reader.read_u16_le()?;

        let compression_method = reader.read_u16_le()?;

        // Last modification time
        reader.read_u16_le()?;

        // Last modification date
        reader.read_u16_le()?;

        // CRC32
        reader.read_u32_le()?;

        // Compressed size
        let compressed_size = reader.read_u32_le()?;

        // Uncompressed size
        reader.read_u32_le()?;

        let name_length = reader.read_u16_le()?;

        let extra_field_length = reader.read_u16_le()?;

        let comment_length = reader.read_u16_le()?;

        // Obsolete field
        reader.read_u16_le()?;

        // Internal file attributes
        reader.read_u16_le()?;

        // External file attributes
        reader.read_u32_le()?;

        // Position of the header from the start of the file
        let header_position = reader.read_u32_le()?;

        let mut file_name = Vec::with_capacity(name_length as usize);
        for _ in 0..name_length {
            file_name.push(reader.read_u8()?);
        }
        let file_name = file_name.into_boxed_slice();

        for _ in 0..extra_field_length {
            reader.read_u8()?;
        }

        for _ in 0..comment_length {
            reader.read_u8()?;
        }

        map.insert(
            file_name,
            FileRecord {
                compression_method: compression_method as usize,
                compressed_size: compressed_size as usize,
                header_position: header_position as usize,
                local_file_header_size: (0x1e + name_length + extra_field_length) as usize,
            },
        );
    }

    Ok(map)
}
