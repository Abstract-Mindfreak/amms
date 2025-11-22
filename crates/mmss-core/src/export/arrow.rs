use arrow2::{
    array::{Array, Int64Array, StringArray, UInt64Array},
    chunk::Chunk,
    datatypes::{DataType, Field, Schema},
    io::ipc::write::{FileWriter, WriteOptions},
};
use std::{fs::File, path::Path};
use crate::structex_bridge::MmssRecord;

pub fn write_records_to_file(path: &Path, records: &[MmssRecord]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    let schema = Schema::from(vec![
        Field::new("id", DataType::UInt64, false),
        Field::new("kind", DataType::Utf8, false),
        Field::new("timestamp", DataType::Int64, false),
        Field::new("payload", DataType::Utf8, false),
    ]);

    let mut writer = FileWriter::try_new(file, &schema, None, Default::default())?;
    let ids: Vec<_> = records.iter().map(|r| r.id).collect();
    let kinds: Vec<_> = records.iter().map(|r| r.kind.as_str()).collect();
    let timestamps: Vec<_> = records.iter().map(|r| r.timestamp).collect();
    let payloads: Vec<_> = records.iter().map(|r| serde_json::to_string(&r.payload).unwrap()).collect();
    let id_array = UInt64Array::from_slice(&ids);
    let kind_array = StringArray::from_slice(kinds);
    let timestamp_array = Int64Array::from_slice(&timestamps);
    let payload_array = StringArray::from_slice(payloads);
    let chunk = Chunk::try_new(vec![
        &id_array as &dyn Array,
        &kind_array,
        &timestamp_array,
        &payload_array,
    ])?;
    writer.write(&chunk, None)?;
    writer.finish()?;
    Ok(())
}
