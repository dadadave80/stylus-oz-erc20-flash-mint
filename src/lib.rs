// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Contracts for Stylus ^0.2.0

#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

use alloc::vec::Vec;
use openzeppelin_stylus::token::erc20::extensions::flash_mint::{
	self, Erc20FlashMint, IErc3156FlashLender
};
use openzeppelin_stylus::token::erc20::{self, Erc20, IErc20};
use stylus_sdk::abi::Bytes;
use stylus_sdk::alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

#[entrypoint]
#[storage]
struct DaveToken {
    erc20: Erc20,
    flash_mint: Erc20FlashMint,
}

#[public]
#[implements(IErc20<Error = flash_mint::Error>, IErc3156FlashLender<Error = flash_mint::Error>)]
impl DaveToken {}

#[public]
impl IErc20 for DaveToken {
    type Error = flash_mint::Error;

    fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    fn balance_of(&self, account: Address) -> U256 {
        self.erc20.balance_of(account)
    }

    fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Self::Error> {
        Ok(self.erc20.transfer(to, value)?)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowance(owner, spender)
    }

    fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Self::Error> {
        Ok(self.erc20.approve(spender, value)?)
    }

    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<bool, Self::Error> {
        Ok(self.erc20.transfer_from(from, to, value)?)
    }
}

#[public]
impl IErc3156FlashLender for DaveToken {
    type Error = flash_mint::Error;

    fn max_flash_loan(&self, token: Address) -> U256 {
        self.flash_mint.max_flash_loan(token, &self.erc20)
    }

    fn flash_fee(&self, token: Address, value: U256) -> Result<U256, Self::Error> {
        Ok(self.flash_mint.flash_fee(token, value)?)
    }

    fn flash_loan(&mut self, receiver: Address, token: Address, value: U256, data: Bytes) -> Result<bool, Self::Error> {
        Ok(self.flash_mint.flash_loan(receiver, token, value, &data, &mut self.erc20)?)
    }
}
