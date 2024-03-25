use std::fmt::format;
use tokio_postgres::{NoTls};
use taos::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let track = get_808_track().await?;
    let alarm_vehicle = get_alarm_vehicle().await?;
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

async fn get_808_track() -> Result<Vec<Record>, Error> {
    let dsn = "taos://127.0.0.1:52006";
    let builder = TaosBuilder::from_dsn(dsn)?;

    let taos = builder.build().await?;

    // Query options 2, use deserialization with serde.
    let database_name = "ivas";
    taos.exec(format!("use {database_name}")).await?;
    let records: Vec<Record> = taos
        .query("select distinct tbname mac_id from `vehicle_track_jt808` where system_time > '2024-03-22 00:00:00' and system_time < '2024-03-23 00:00:00'")
        .await?
        .deserialize()
        .try_collect()
        .await?;

    Ok(records)
}

async fn get_alarm_vehicle() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect("host=localhost port=52001 \
    user=ivas_data_exc_terminal dbname=ivas password=45oudnm6GUXy5kFW", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    let mut vehicle_list = Vec::new();
    for row in client.query("SELECT distinct plate_no, plate_color FROM vd_his_terminal_alarm a
        left join bi_inf_vehicle_local l on a.vehicle_id = l.id
        where a.beg_sys_time >
    '2024-03-22 00:00:00' and a.beg_sys_time < '2024-03-23 00:00:00'",
                            &[]).await? {
        let plate_no: String = row.get(0);
        let plate_color: i32 = row.get(1);
        vehicle_list.push(format!("VT_{}_{}", plate_color, plate_no));
    }
    Ok(vehicle_list)
}