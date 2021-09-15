use serde_json::Value;
use sp_arithmetic::per_things::Permill;

#[test]
fn deserialize_worked() {
	let data = r#"{
        "data": {
            "id": "polkadot",
            "rank": "8",
            "symbol": "DOT",
            "name": "Polkadot",
            "supply": "1028848869.2290900000000000",
            "maxSupply": null,
            "marketCapUsd": "32468150830.5892038555409109",
            "volumeUsd24Hr": "960260748.3748634842797382",
            "priceUsd": "31.5577455558826492",
            "changePercent24Hr": "8.6645884179153715",
            "vwap24Hr": "30.9966588686600216",
            "explorer": "https://polkascan.io/polkadot"
        },
        "timestamp": 1631427531000
    }"#;

	// Parse the string of data into serde_json::Value.
	let v: Value = serde_json::from_str(data).unwrap();

	let dot_price: &str = v["data"]["priceUsd"].as_str().unwrap();
	assert_eq!(dot_price, "31.5577455558826492");

	let price_parts: Vec<&str> = dot_price.split(".").collect();
	let left_part: u64 = price_parts[0].parse::<u64>().unwrap();
	let right_part: Permill = Permill::from_parts(price_parts[1][0..6].parse::<u32>().unwrap());

	assert_eq!((left_part, right_part), (31, Permill::from_parts(557745 as u32)));
}
