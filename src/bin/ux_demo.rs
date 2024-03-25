use postgres::{Client, NoTls};

fn main() {
    let mut client = Client::connect("host=localhost port=52001 user=ivas_data_exc_terminal dbname=ivas password=45oudnm6GUXy5kFW", NoTls).unwrap();

    for row in client.query("SELECT plate_no FROM bi_inf_vehicle_local", &[]).unwrap() {
        let palte_no: &str = row.get(0);

        println!("found vehicle: {:?}", palte_no);
    }
}
