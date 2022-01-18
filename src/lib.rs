use serde::{Deserialize, Serialize};
use std::os::raw::c_int;
use tarantool::index::IteratorType;
use tarantool::space;
use tarantool::tuple::{AsTuple, FunctionArgs, FunctionCtx, Tuple};

#[no_mangle]
pub extern "C" fn luaopen_tmcrud(_l: std::ffi::c_void) -> c_int {
    // Tarantool calls this function upon require("easy")
    println!("easy module loaded");
    0
}

#[no_mangle]
pub extern "C" fn tmcrud(_: FunctionCtx, _: FunctionArgs) -> c_int {
    println!("hello world");
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