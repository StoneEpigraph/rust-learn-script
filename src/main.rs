use tokio_postgres::types::ToSql;
use tokio_postgres::{NoTls};
use taos::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let search_start_date_str = "2024-03-23 00:00:00";
    let search_end_date_str = "2024-03-25 00:00:00";
    let track = get_808_track(search_start_date_str, search_end_date_str).await?;
    let alarm_vehicle = get_alarm_vehicle(search_start_date_str, search_end_date_str).await?;
    println!("track length: {}, alarm_vehicle length: {}", &track.len(), &alarm_vehicle.len());
    let track_mac_ids = track.into_iter().map(|x| x.mac_id).collect::<Vec<String>>();
    for plate_no in alarm_vehicle.iter() {
        if (!track_mac_ids.contains(plate_no)) {
            println!("{}", plate_no);
        }
    }
    Ok(())
}


#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Record {
    // deserialize timestamp to chrono::DateTime<Local>
    mac_id: String,
}

async fn get_808_track(start_date_str: &str, end_date_str: &str) -> Result<Vec<Record>, Error> {
    let dsn = "taos://127.0.0.1:52006";
    let builder = TaosBuilder::from_dsn(dsn)?;

    let taos = builder.build().await?;

    // Query options 2, use deserialization with serde.
    let database_name = "ivas";
    taos.exec(format!("use {database_name}")).await?;
    let records: Vec<Record> = taos
        .query(format!("select distinct tbname mac_id from `vehicle_track_jt808` where system_time > '{}' and system_time < '{}'", start_date_str, end_date_str))
        .await?
        .deserialize()
        .try_collect()
        .await?;

    Ok(records)
}

async fn get_alarm_vehicle<'a>(start_date_str: &'a str, end_date_str: &'a str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect("host=localhost port=52001 \
    user=ivas_data_exc_terminal dbname=ivas password=45oudnm6GUXy5kFW", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    let mut vehicle_list = Vec::new();
    for row in client.query(format!("SELECT distinct plate_no, plate_color FROM vd_his_terminal_alarm a
        left join bi_inf_vehicle_local l on a.vehicle_id = l.id
        where a.beg_sys_time > '{start_date_str}' and a.beg_sys_time < '{end_date_str}'").as_str(),
                                        &[]).await? {
        let plate_no: String = row.get(0);
        let plate_color: i32 = row.get(1);
        vehicle_list.push(format!("VT_{}_{}", plate_color, plate_no));
    }
    Ok(vehicle_list)
}