use serde::{Deserialize, Serialize};
use std::os::raw::c_int;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tarantool::index::IteratorType;
use tarantool::space;
use tarantool::tuple::{AsTuple, FunctionArgs, FunctionCtx, Tuple};
use actix_web::{rt::System, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use tarantool::fiber::time;
use tarantool::net_box;
use tarantool::net_box::Conn;

#[get("/forecast/all")]
async fn hello() -> Result<impl Responder> {
    //let f = read_all_forecasts();
   // println!("{:?}", f);

    // read_all_forecasts();
    //
    // Ok("web::Json(f)")

    //let res = conn.call("read", &(), &Default::default());

    //let data = res.unwrap().unwrap();
    //println!("{:?}", data.str);

    Ok("ok")
}

//
//
//
// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }
//
// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

struct AppState {
    conn: Conn,
}

#[no_mangle]
pub extern "C" fn luaopen_tmcrud(_l: std::ffi::c_void) -> c_int {
    // let conn = net_box::Conn::new("localhost:3301", net_box::ConnOptions::default(), None).unwrap();
    // let conn = Arc::new(Mutex::new(conn));

    // thread::spawn(move || {
    //     sleep(Duration::from_secs(2));
    //     let conn = net_box::Conn::new("localhost:3301", net_box::ConnOptions::default(), None).unwrap();
    //
    //     // let sys = System::new("http-server");
    //     //
    //     // let srv = HttpServer::new(|| {
    //     //     App::new()
    //     //         // .data(conn.clone())
    //     //         .route("/", web::get().to(|| HttpResponse::Ok()))
    //     //         .service(hello)
    //     // })
    //     //     .bind("127.0.0.1:8080")?
    //     //     .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
    //     //     .run();
    //     //
    //     // sys.run()
    // });

    println!("tmcrud module loaded");
    0
}

#[no_mangle]
pub extern "C" fn tmcrud(_: FunctionCtx, _: FunctionArgs) -> c_int {
    println!("hello world");

    thread::spawn(move || {
        sleep(Duration::from_secs(2));
        let conn = net_box::Conn::new("localhost:3301", net_box::ConnOptions::default(), None).unwrap();
    });

    0
}

#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    pub city_id: i32,
    pub ts: i32,
    pub temp: f64,
    pub pressure: f64,
}

impl AsTuple for Forecast {}

#[no_mangle]
pub extern "C" fn read(_: FunctionCtx, _: FunctionArgs) -> c_int {
    let fcst = read_all_forecasts();
    println!("forecasts = {:?}", fcst);

    0
}

#[derive(Serialize, Deserialize, Debug)]
struct ReadAtArgs {
    pub city_id: i32,
    pub ts: i32,
}

impl AsTuple for ReadAtArgs {}

#[no_mangle]
pub extern "C" fn read_at(_: FunctionCtx, args: FunctionArgs) -> c_int {
    let args: Tuple = args.into();
    let args = args.into_struct::<ReadAtArgs>().unwrap();

    match read_forecast(args.city_id, args.ts) {
        None => println!("forecast not found for date"),
        Some(f) => println!("forecast = {:?}", f),
    }

    0
}

#[no_mangle]
pub extern "C" fn insert(ctx: FunctionCtx, args: FunctionArgs) -> c_int {
    let args: Tuple = args.into();
    let new_forecast = args.into_struct::<Forecast>().unwrap();

    let res = insert_forecast(&new_forecast);

    ctx.return_tuple(&res.unwrap().unwrap()).unwrap()
}

fn read_all_forecasts() -> Vec<Forecast> {
    let space = space::Space::find("forecast_city").unwrap();

    let forecasts = space.select(IteratorType::All, &()).unwrap();

    forecasts.map(|t| t.into_struct::<Forecast>().unwrap()).collect()
}

fn read_forecast(city_id: i32, ts: i32) -> Option<Forecast> {
    let space = space::Space::find("forecast_city").unwrap();

    let key = (city_id, ts);

    match space.get(&key).unwrap() {
        None => None,
        Some(tuple) => Some(tuple.into_struct::<Forecast>().unwrap())
    }
}

fn insert_forecast(fcst: &Forecast) -> Result<Option<Tuple>, tarantool::error::Error> {
    let mut space = space::Space::find("forecast_city").unwrap();

    space.insert(fcst)
}