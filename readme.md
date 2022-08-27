# Install

Install Rust:

https://www.rust-lang.org/tools/install

Clone this repository:

```
git clone https://github.com/keteik/rust-api-task.git
```

Go to project directory:

```
cd rust-api-task
```

Build project for dev:

```
cargo build
```

Build project for release:

```
cargo build --release
```

Run project:

```
cargo run
```

# Endpoints

## /calculateDisselUsageForDistance
**http://localhost:3000/calculateDisselUsageForDistance?distance=327&yearOfProduction=1997&fuelUsagePer100KM=7**

```
curl 
	--location 
	--request GET 'http://localhost:3000/calculateDisselUsageForDistance?distance=327&yearOfProduction=1997&fuelUsagePer100KM=7'
```

## /probabilityOfUnitInjectorFail
**http://localhost:3000/probabilityOfUnitInjectorFail?VIN=4Y1SL65848Z411439**


Only POST reqests are accepted. Email and password fields in JSON format are required in body:

```
curl 
	--location 
	--request GET 'http://localhost:3000/probabilityOfUnitInjectorFail?VIN=4Y1SL65848Z411439'
```

# Notes
This project was tested with the Postman.