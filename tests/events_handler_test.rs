use rust_decimal::Decimal;
use std::str::FromStr;
use tokio::fs;
use uniswap_dai_usd_monitor::events_handler::EventsHandler;
use uniswap_dai_usd_monitor::setup_web3;
use uniswap_dai_usd_monitor::swap_details::SwapDirection;
use web3::ethabi::Address;
use web3::types::U256;
use web3::{ethabi, ethabi::Token, types::BlockHeader};

async fn load_fixtures() -> Vec<BlockHeader> {
    let json = fs::read_to_string("tests/fixtures/block_headers.json")
        .await
        .expect("Failed to read fixtures file");
    serde_json::from_str(&json).expect("Failed to parse blocks")
}

fn extract_param_by_name(parsed_log: &ethabi::Log, name: &str) -> Option<Token> {
    let token = parsed_log
        .params
        .iter()
        .find(|p| p.name == name)
        .map(|p| p.value.clone());
    token
}

#[tokio::test]
async fn test_handle_events_21836327() {
    let headers = load_fixtures().await;
    let block_number = headers[0].number.unwrap();
    assert_eq!(21836327_u64, block_number.as_u64());

    let web3 = setup_web3().await.unwrap();
    let events_handler = EventsHandler::new(web3).unwrap();

    // fetch_swap_logs
    let block_hash = headers[0].hash.unwrap();
    let raw_logs = events_handler.fetch_swap_logs(block_hash).await.unwrap();
    println!("raw_logs: {:?}", raw_logs);
    assert_eq!(raw_logs.len(), 1);

    // parse_logs
    let parsed_logs = events_handler.parse_logs(raw_logs).unwrap();
    assert_eq!(parsed_logs.len(), 1);
    let parsed_log = parsed_logs[0].clone();
    println!("{:#?}", parsed_log);

    // let's make some assertions about parsed log (as it will be used further in matching swap info)
    // the values come from debug output of `parsed_log`
    let param = extract_param_by_name(&parsed_log, "sender").unwrap();
    assert_eq!(
        param,
        Token::Address(
            "0x000000000c56e91f092023d942aee89b3cc089ff"
                .parse()
                .unwrap()
        )
    );

    let param = extract_param_by_name(&parsed_log, "recipient").unwrap();
    assert_eq!(
        param,
        Token::Address(
            "0xe0554a476a092703abdb3ef35c80e0d76d32939f"
                .parse()
                .unwrap()
        )
    );

    let param = extract_param_by_name(&parsed_log, "amount0").unwrap();
    assert_eq!(param, Token::Int(3435661580949251204399_i128.into()));

    let param = extract_param_by_name(&parsed_log, "amount1").unwrap();
    assert_eq!(
        param,
        Token::Int(
            U256::from_dec_str(
                "115792089237316195423570985008687907853269984665640564039457584007909694262531"
            )
            .unwrap()
        )
    );

    let swap_info = events_handler.to_swap_details(parsed_logs).await.unwrap();
    println!("{:#?}", swap_info);

    assert_eq!(swap_info.len(), 1);
    let swap_details = &swap_info[0];

    let parsed_result: Result<Address, _> = "0x000000000c56e91f092023d942aee89b3cc089ff".parse();
    assert_eq!(swap_details.sender, parsed_result.unwrap());

    let parsed_result: Result<Address, _> = "0xe0554a476a092703abdb3ef35c80e0d76d32939f".parse();
    assert_eq!(swap_details.recipient, parsed_result.unwrap());
    assert_eq!(
        swap_details.amount0_raw,
        3435661580949251204399_i128.into() // from i128 to Int
    );
    // DAI uses a precision of `10^-18` (1/10^18)
    // 3435661580949251204399 / 1_000_000_000_000_000_000 = 3435.66158095  (from wei to DAI)
    assert_eq!(
        swap_details.amount0_as_decimal_num.round_dp(8), // the number of decimal points to round to
        Decimal::from_str("3435.66158095").unwrap()
    );

    assert_eq!(
        swap_details.amount1_raw.to_string().as_str(),
        "115792089237316195423570985008687907853269984665640564039457584007909694262531"
    );
    // INFO! Can use Python shell for such conversions
    // amount1 = 115792089237316195423570985008687907853269984665640564039457584007909694262531
    // max = U256::max_value() = 2^256 - 1 => 115792089237316195423570985008687907853269984665640564039457584007913129639935
    // max_half = U256::max_value() / 2 = 115792089237316195423570985008687907853269984665640564039457584007913129639935 / 2 => 57896044618658097711785492504343953926634992332820282019728792003956564819967
    // amount1 > max_half, so yes, it's negative
    // positive = (max + 1) - amount1 => 115792089237316195423570985008687907853269984665640564039457584007913129639935 + 1 - 115792089237316195423570985008687907853269984665640564039457584007909694262531
    // positive = 3435377405 (our positive number)
    // negative = -3435377405
    // convert to USDC decimals (divide by 10^6 because USDC has 6 decimal places) (-3435377405 / 1000000 = -3435.377405)
    assert_eq!(
        swap_details.amount1_as_decimal_num,
        Decimal::from_str("-3435.377405").unwrap()
    );

    // since amount1 is negative & it denotes USDC, it's DAI -> USDC
    assert_eq!(swap_details.direction, SwapDirection::DaiToUsdc);
}

#[tokio::test]
async fn test_handle_events_21904546() {
    let json = fs::read_to_string("tests/fixtures/21904546/block_headers.json")
        .await
        .expect("Failed to read fixtures file");
    let headers: Vec<BlockHeader> = serde_json::from_str(&json).expect("Failed to parse blocks");

    let block_number = headers[0].number.unwrap();
    assert_eq!(21904546_u64, block_number.as_u64());

    let web3 = setup_web3().await.unwrap();
    let events_handler = EventsHandler::new(web3).unwrap();

    // fetch_swap_logs
    let block_hash = headers[0].hash.unwrap();
    let raw_logs = events_handler.fetch_swap_logs(block_hash).await.unwrap();
    println!("raw_logs: {:#?}", raw_logs);
    assert_eq!(raw_logs.len(), 1);

    // parse_logs, these parsed values come from external crate ethabi,
    // so here, we simply reply on it & actual assertions are made against our SwapDetails implementation
    let parsed_logs = events_handler.parse_logs(raw_logs).unwrap();
    assert_eq!(parsed_logs.len(), 1);
    let parsed_log = parsed_logs[0].clone();
    println!("{:#?}", parsed_log);

    // let's make some assertions about parsed log (as it will be used further in matching swap info)
    // the values come from debug output of `parsed_log`
    let param = extract_param_by_name(&parsed_log, "sender").unwrap();
    assert_eq!(
        param,
        Token::Address(
            "0x4347b972898b2fd780adbdaa29b4a5160a9f4fe5"
                .parse()
                .unwrap()
        )
    );

    let param = extract_param_by_name(&parsed_log, "recipient").unwrap();
    assert_eq!(
        param,
        Token::Address(
            "0x4304718165a17091b9e039815ae025d9ec151f31"
                .parse()
                .unwrap()
        )
    );

    let param = extract_param_by_name(&parsed_log, "amount0").unwrap();
    assert_eq!(
        param,
        Token::Int(
            U256::from_dec_str(
                "115792089237316195423570985008687907853269984665640564020856256472406180424393"
            )
            .unwrap()
        )
    );

    let param = extract_param_by_name(&parsed_log, "amount1").unwrap();
    assert_eq!(
        param,
        Token::Int(U256::from_dec_str("18602732366").unwrap())
    );

    let swap_info = events_handler.to_swap_details(parsed_logs).await.unwrap();
    println!("{:#?}", swap_info);

    assert_eq!(swap_info.len(), 1);
    let swap_details = &swap_info[0];

    let parsed_result: Result<Address, _> = "0x4347b972898b2fd780adbdaa29b4a5160a9f4fe5".parse();
    assert_eq!(swap_details.sender, parsed_result.unwrap());

    let parsed_result: Result<Address, _> = "0x4304718165a17091b9e039815ae025d9ec151f31".parse();
    assert_eq!(swap_details.recipient, parsed_result.unwrap());

    // no conversion (same as parsed_log)
    assert_eq!(
        swap_details.amount0_raw,
        U256::from_dec_str(
            "115792089237316195423570985008687907853269984665640564020856256472406180424393"
        )
        .unwrap()
    );
    // let's see how this got converted to `-18601.32753551`
    // step 1: is it negative ? (> max/2)
    // 115792089237316195423570985008687907853269984665640564020856256472406180424393 > 57896044618658097711785492504343953926634992332820282019728792003956564819967 => true
    // positive_amount = max - raw + 1 => 115792089237316195423570985008687907853269984665640564039457584007913129639935 - 115792089237316195423570985008687907853269984665640564020856256472406180424393 + 1
    // positive_amount = 18601327535506949215543 (can verify on Python shell)
    // to_negative = -18601327535506949215543
    // result = -18601.32753550695 (-18601327535506949215543 / 1_000_000_000_000_000_000) (10^-18 = 1/10^18 = 1/1_000_000_000_000_000_000)
    assert_eq!(
        swap_details.amount0_as_decimal_num.round_dp(8), // the number of decimal points to round to
        Decimal::from_str("-18601.32753551").unwrap()
    );

    assert_eq!(swap_details.amount1_raw, 18602732366_i64.into());
    // 18602732366 / 1_000_000 => 18602.732366
    assert_eq!(
        swap_details.amount1_as_decimal_num.round_dp(8),
        Decimal::from_str("18602.732366").unwrap()
    );

    // since `amount0` denotes DAI & it's negative
    // the negative indicates the amount output to the `receiver` address
    assert_eq!(swap_details.direction, SwapDirection::UsdcToDai);
}
