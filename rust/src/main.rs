use rocket::serde::{json::Json, Serialize};
use sysinfo::System;
use sysinfo::SystemExt;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RandomAccessMemoryStats {
    total: u64,
    swap: u64,
    free: u64,
    used: u64,
}

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Json<RandomAccessMemoryStats> {
let mut sys = System::new_all();
    sys.refresh_all();

    Json(RandomAccessMemoryStats {
        total: sys.total_memory(),
        swap: sys.total_swap(),
        free: sys.free_memory(),
        used: sys.used_memory(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
