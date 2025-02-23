use anyhow::anyhow;
use log::debug;
use rust_decimal::Decimal;
use std::str::FromStr;
use web3::{
    ethabi,
    ethabi::Int,
    types::{Address, U256},
};

#[derive(Debug, PartialEq, Eq)]
pub enum SwapDirection {
    DaiToUsdc,
    UsdcToDai,
}

#[derive(Debug)]
pub struct SwapDetails {
    pub sender: Address,
    pub recipient: Address,
    pub amount0_raw: Int,
    pub amount0_as_decimal_num: Decimal,
    pub amount1_raw: Int,
    pub amount1_as_decimal_num: Decimal,
    /// Direction of the swap depends on one of the values being negative.
    /// The negative indicates the amount output to the `receiver` address.
    /// e.g., 1000 `amount0`/DAI and -50 `amount1`/USDC indicates a swap direction of DAI -> USDC
    pub direction: SwapDirection,
}

impl SwapDetails {
    /// So while U256 can't store negative numbers directly, the contract uses two's complement encoding to
    /// represent negative values within the positive number space. The function detects this encoding and
    /// converts it back to a proper negative number string representation.
    fn format_amount(amount: U256) -> String {
        let max = U256::max_value();
        // is it negative ?
        if amount > max / 2 {
            // handle negative numbers in two's complement
            // convert from two's complement to get positive value
            let positive_amount = max - amount + U256::from(1);
            // add negative sign
            format!("-{}", positive_amount)
        } else {
            amount.to_string()
        }
    }

    /// Converts raw amount from contract to decimal DAI amount
    fn format_amount0(amount0: U256) -> Decimal {
        let amount_str = Self::format_amount(amount0);
        let dai_amount = Decimal::from_str(&amount_str).unwrap_or(Decimal::ZERO);
        let divisor = Decimal::from_str("1000000000000000000").unwrap(); // 1e18
        dai_amount / divisor
    }

    /// Converts raw amount from contract to decimal USDC amount
    fn format_amount1(amount1: U256) -> Decimal {
        let amount_str = Self::format_amount(amount1);
        let usdc_amount = Decimal::from_str(&amount_str).unwrap_or(Decimal::ZERO);
        let divisor = Decimal::from_str("1000000").unwrap(); // 1e6 for USDC
        usdc_amount / divisor
    }

    pub fn from_parsed_log(parsed_log: ethabi::Log) -> Result<SwapDetails, anyhow::Error> {
        debug!("parsed log: {:#?}", parsed_log);

        let amount0 = Self::extract_param_by_name(&parsed_log, "amount0")?
            .into_int() // This gives us ethabi::Token::Int
            .ok_or(anyhow!("Invalid type: expected Int"))?;

        let amount1 = Self::extract_param_by_name(&parsed_log, "amount1")?
            .into_int()
            .ok_or(anyhow!("Invalid type: expected Int"))?;

        let amount0_decimal = Self::format_amount0(amount0);
        let amount1_decimal = Self::format_amount1(amount1);

        let swap_details = SwapDetails {
            sender: Self::extract_param_by_name(&parsed_log, "sender")? // Result -> Token
                .into_address()
                // convert Option<Address> to Result<Address, & then ? will unwrap it's value
                .ok_or(anyhow!("Invalid type: expected Address"))?,
            recipient: Self::extract_param_by_name(&parsed_log, "recipient")?
                .into_address()
                .ok_or(anyhow!("Invalid type: expected Address"))?,

            amount0_raw: amount0,
            amount0_as_decimal_num: amount0_decimal,
            amount1_raw: amount1,
            amount1_as_decimal_num: amount1_decimal,
            direction: if amount1_decimal < Decimal::ZERO {
                SwapDirection::DaiToUsdc
            } else {
                SwapDirection::UsdcToDai
            },
        };

        Ok(swap_details)
    }

    /// This is more reliable than extracting by index (as indexes could change in future)
    fn extract_param_by_name(
        parsed_log: &ethabi::Log,
        name: &str,
    ) -> Result<ethabi::Token, anyhow::Error> {
        let token = parsed_log
            .params
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.value.clone())
            .ok_or_else(|| anyhow!("Parameter {} not found", name))?;

        Ok(token)
    }
}
