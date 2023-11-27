use crate::dat::{Config, Reader};
use crate::model::{ClassRecord, InstructorRecord, MeetingRecord, ToRow};
use sqlx::{prelude::*, Pool, Postgres, QueryBuilder};
use std::io::BufReader;

const BIND_LIMIT: usize = 65535;

pub async fn import(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let mut connection = pool.acquire().await?;
    connection
        .transaction(|transaction| {
            Box::pin(async move {
                macro_rules! read_data {
                    ($filename:expr, $table:expr, $record:ty) => {{
                        let keys = <$record>::keys();
                        let field_count = keys.len();
                        let mut file = std::fs::File::open($filename).expect("should open file");
                        let mut reader = Reader::new(
                            BufReader::new(&mut file),
                            Config::new(field_count as u32, b'|'),
                        );
                        let mut csv_reader = csv::ReaderBuilder::new()
                            .has_headers(false)
                            .delimiter(b'|')
                            .flexible(true)
                            .from_reader(&mut reader);

                        let records = csv_reader
                            .deserialize::<$record>()
                            .filter_map(Result::ok)
                            .collect::<Vec<$record>>();

                        let keys = keys.join(", ");
                        let table = $table;
                        let stem = format!("INSERT INTO {table}({keys}) ");

                        for chunk in records.chunks(BIND_LIMIT / field_count) {
                            let mut query_builder: QueryBuilder<Postgres> =
                                QueryBuilder::new(&stem);
                            query_builder.push_values(chunk, |mut builder, entry| {
                                entry.bind(&mut builder);
                            });
                            let query = query_builder.build();
                            query.execute(&mut **transaction).await?;
                        }
                    }};
                }

                sqlx::query("DELETE FROM classes")
                    .execute(&mut **transaction)
                    .await?;

                read_data!("./data/cshclass.dat", "classes", ClassRecord);
                read_data!("./data/cshmtgpat.dat", "meetings", MeetingRecord);
                read_data!("./data/cshinstr.dat", "instructors", InstructorRecord);

                Ok(()) as Result<(), sqlx::Error>
            })
        })
        .await?;
    Ok(())
}
