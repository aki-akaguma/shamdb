use siamesedb::filedb::{CheckFileDbMap, FileBufSizeParam, FileDbParams};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "-s" => check_string(&args[2])?,
        "-b" => check_bytes(&args[2])?,
        "-u" => check_u64(&args[2])?,
        _ => {
            eprintln!("[usage] {} {{-s|-b|-u}} path", args[0]);
        }
    }
    Ok(())
}

#[derive(Debug, Default, Clone, Copy)]
struct CheckC {
    check: bool,
    f_mst: bool,
    f_graph: bool,
}

fn check_string(db_name: &str) -> std::io::Result<()> {
    let db = siamesedb::open_file(db_name).unwrap();
    let db_map = db
        .db_map_string_with_params(
            "some_map1",
            FileDbParams {
                dat_buf_size: FileBufSizeParam::PerMille(1000),
                idx_buf_size: FileBufSizeParam::PerMille(1000),
            },
        )
        .unwrap();
    _print_check_db_map(
        &db_map,
        CheckC {
            check: true,
            f_mst: true,
            f_graph: false,
            /*
            check: false,
            f_mst: false,
            f_graph: true,
            */
        },
    );
    //
    Ok(())
}

fn check_bytes(db_name: &str) -> std::io::Result<()> {
    let db = siamesedb::open_file(db_name).unwrap();
    let db_map = db
        .db_map_bytes_with_params(
            "some_map1",
            FileDbParams {
                dat_buf_size: FileBufSizeParam::PerMille(1000),
                idx_buf_size: FileBufSizeParam::PerMille(1000),
            },
        )
        .unwrap();
    _print_check_db_map(
        &db_map,
        CheckC {
            check: true,
            f_mst: true,
            f_graph: false,
            /*
            check: false,
            f_mst: false,
            f_graph: true,
            */
        },
    );
    //
    Ok(())
}

fn check_u64(db_name: &str) -> std::io::Result<()> {
    let db = siamesedb::open_file(db_name).unwrap();
    let db_map = db
        .db_map_u64_with_params(
            "some_map1",
            FileDbParams {
                dat_buf_size: FileBufSizeParam::PerMille(1000),
                idx_buf_size: FileBufSizeParam::PerMille(1000),
            },
        )
        .unwrap();
    _print_check_db_map(
        &db_map,
        CheckC {
            check: true,
            f_mst: true,
            f_graph: false,
            /*
            check: false,
            f_mst: false,
            f_graph: true,
            */
        },
    );
    //
    Ok(())
}

fn _print_check_db_map(db_map: &dyn CheckFileDbMap, check_cnf: CheckC) {
    if check_cnf.f_graph {
        println!("{}", db_map.graph_string_with_key_string().unwrap());
    }
    if check_cnf.check {
        //
        println!("record free: {:?}", db_map.count_of_free_record().unwrap());
        let (rec_v, node_v) = db_map.count_of_used_node().unwrap();
        println!("record used: {:?}", rec_v);
        println!("node free: {:?}", db_map.count_of_free_node().unwrap());
        println!("node used: {:?}", node_v);
        //
        println!(
            "db_map.depth_of_node_tree(): {}",
            db_map.depth_of_node_tree().unwrap()
        );
        println!("db_map.is_balanced(): {}", db_map.is_balanced().unwrap());
        println!("db_map.is_dense(): {}", db_map.is_dense().unwrap());
        #[cfg(feature = "buf_stats")]
        println!("db_map.buf_stats(): {:?}", db_map.buf_stats());
        //
        println!(
            "record_size_stats(): {}",
            db_map.record_size_stats().unwrap()
        );
        println!("keys_count_stats(): {}", db_map.keys_count_stats().unwrap());
        println!("key_length_stats(): {}", db_map.key_length_stats().unwrap());
        println!(
            "value_length_stats(): {}",
            db_map.value_length_stats().unwrap()
        );
    }
    if check_cnf.f_mst {
        println!("db_map.is_mst_valid(): {}", db_map.is_mst_valid().unwrap());
    }
}
