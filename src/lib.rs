use std::{ fs::File, io::{ Read, Seek, SeekFrom, Write }, path::Path, sync::{ Arc, RwLock } };
#[macro_export]
macro_rules! json {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
        let mut s = String::new();
        s.push('{');

        $(
            let key = $key;
            let value = $value;
            s.push_str(&format!(r#""{}": "{}""#,key, value));
        )*
        if s.ends_with(','){
            s.pop();
        }
        s.push('}');
        s
        }
    };
}

pub struct JacsDb {
    file: Arc<RwLock<File>>,
}

impl JacsDb {
    pub fn new(db_name: String) -> Self {
        let file_path = Path::new(&db_name);
        if !file_path.exists() {
            File::create(&db_name).unwrap();
        }
        Self {
            file: Arc::new(
                RwLock::new(
                    File::options()
                        .read(true)
                        .write(true)
                        .append(true)
                        .open(file_path)
                        .expect("Error When try to set file")
                )
            ),
        }
    }

    fn serializing(content: String) -> Vec<u8> {
        let content: String = content
            .chars()
            .filter(|x| !x.is_whitespace())
            .collect();
        let mut bson_data: Vec<u8> = vec![0xff, 0, 0, 0, 0];
        let byte_key: u8 = 0b10110110;
        let mut content_bytes: Vec<u8> = content.as_bytes().to_vec();
        let byte_shift = (content_bytes.len() % 8) + 1;
        content_bytes.iter_mut().for_each(|x| {
            *x =
                x
                    .rotate_left(byte_shift.try_into().unwrap())
                    .wrapping_add(byte_shift.try_into().unwrap()) ^ byte_key;
        });

        bson_data.extend_from_slice(&content_bytes);
        let data_length: u32 = content_bytes.len() as u32;
        bson_data[1..5].copy_from_slice(&data_length.to_le_bytes());
        bson_data
    }

    fn deserializing(buffer_data: Vec<u8>) -> Vec<String> {
        let byte_key: u8 = 0b10110110;
        let mut index = 0_usize;
        let mut final_data: Vec<String> = Vec::new();
        while index < buffer_data.len() {
            if buffer_data[index] == 0xff {
                let data_length = u32::from_le_bytes(
                    buffer_data[index + 1..index + 5].try_into().unwrap()
                );
                index += 5;
                let data_end_index = data_length as usize;
                let mut decrypt_data: Vec<u8> = buffer_data[index..index + data_end_index]
                    .try_into()
                    .unwrap();
                let byte_shift = (data_length % 8) + 1;
                decrypt_data.iter_mut().for_each(|x: &mut u8| {
                    *x ^= byte_key;
                    *x = x
                        .wrapping_sub(byte_shift.try_into().unwrap())
                        .rotate_right(byte_shift.try_into().unwrap());
                });

                let data: &str = std::str::from_utf8(&decrypt_data).unwrap();
                final_data.push(data.to_string());
                index += data_end_index - 1;
            }
            index += 1;
        }
        final_data
    }

    pub fn create_one(&self, content: String) {
        let byte_data = Self::serializing(content);
        let binding = Arc::clone(&self.file);
        let mut file = binding.write().unwrap();
        file.write_all(&byte_data).unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
    }
    pub fn read_all(&self) -> Vec<String> {
        let binding = Arc::clone(&self.file);
        let file = binding.read().unwrap();
        let mut buffer_data: Vec<u8> = Vec::new();

        if let Err(e) = (&*file).read_to_end(&mut buffer_data) {
            eprintln!("Failed to read file: {}", e);
            return vec![];
        }
        Self::deserializing(buffer_data)
    }
}
