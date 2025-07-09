# WeatherDummy

Dummy REST endpoint that returns random weather data for a location

The project is written in Rust and uses Rocket.

## How to run

Make sure you have Rust installed.

Update dependencies:

```
cargo update
```

Run the project:

```
cargo run
```

## How to use

If the server is running you can use the following endpoints:

`localhost:8000/weather/Nuremburg`

This returns

```
{"location":"Nuremburg","temperature":16.664558,"description":"Cloudy"}
```
