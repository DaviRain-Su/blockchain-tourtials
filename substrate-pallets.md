# Substrate Pallets

# Assets Module

A simple, secure module for dealing with fungible assets.

## Overview

The Assets module provides functionality for asset management of fungible asset classes with a fixed supply, including:

- Asset Issuance
- Asset Transfer
- Asset Destruction

To use it in your runtime, you need to implement the assets [`assets::Trait`](https://docs.rs/pallet-assets/latest/pallet_assets/trait.Trait.html).

The supported dispatchable functions are documented in the [`assets::Call`](https://docs.rs/pallet-assets/latest/pallet_assets/enum.Call.html) enum.

### Terminology

- **Asset issuance:** The creation of a new asset, whose total supply will belong to the account that issues the asset.
- **Asset transfer:** The action of transferring assets from one account to another.
- **Asset destruction:** The process of an account removing its entire holding of an asset.
- **Fungible asset:** An asset whose units are interchangeable.
- **Non-fungible asset:** An asset for which each unit has unique characteristics.

### Goals

The assets system in Substrate is designed to make the following possible:

- Issue a unique asset to its creator's account.
- Move assets between accounts.
- Remove an account's balance of an asset when requested by that account's owner and update the asset's total supply.

## Interface

### Dispatchable Functions

- `issue` - Issues the total supply of a new fungible asset to the account of the caller of the function.
- `transfer` - Transfers an `amount` of units of fungible asset `id` from the balance of the function caller's account (`origin`) to a `target` account.
- `destroy` - Destroys the entire holding of a fungible asset `id` associated with the account that called the function.

Please refer to the [`Call`](https://docs.rs/pallet-assets/latest/pallet_assets/enum.Call.html) enum and its associated variants for documentation on each function.

### Public Functions

- `balance` - Get the asset `id` balance of `who`.
- `total_supply` - Get the total supply of an asset `id`.

Please refer to the [`Module`](https://docs.rs/pallet-assets/latest/pallet_assets/struct.Module.html) struct for details on publicly available functions.

## Usage

The following example shows how to use the Assets module in your runtime by exposing public functions to:

- Issue a new fungible asset for a token distribution event (airdrop).
- Query the fungible asset holding balance of an account.
- Query the total supply of a fungible asset that has been issued.

### Prerequisites

Import the Assets module and types and derive your runtime's configuration traits from the Assets module trait.

### Simple Code Snippet

```rust
use pallet_assets as assets;
use frame_support::{decl_module, dispatch, ensure};
use frame_system::ensure_signed;

pub trait Config: assets::Config { }

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		pub fn issue_token_airdrop(origin) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin).map_err(|e| e.as_str())?;

			const ACCOUNT_ALICE: u64 = 1;
			const ACCOUNT_BOB: u64 = 2;
			const COUNT_AIRDROP_RECIPIENTS: u64 = 2;
			const TOKENS_FIXED_SUPPLY: u64 = 100;

			ensure!(!COUNT_AIRDROP_RECIPIENTS.is_zero(), "Divide by zero error.");

			let asset_id = Self::next_asset_id();

			<NextAssetId<T>>::mutate(|asset_id| *asset_id += 1);
			<Balances<T>>::insert((asset_id, &ACCOUNT_ALICE), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
			<Balances<T>>::insert((asset_id, &ACCOUNT_BOB), TOKENS_FIXED_SUPPLY / COUNT_AIRDROP_RECIPIENTS);
			<TotalSupply<T>>::insert(asset_id, TOKENS_FIXED_SUPPLY);

			Self::deposit_event(RawEvent::Issued(asset_id, sender, TOKENS_FIXED_SUPPLY));
			Ok(())
		}
	}
}
```

## Assumptions

Below are assumptions that must be held when using this module. If any of them are violated, the behavior of this module is undefined.

- The total count of assets should be less than `Config::AssetId::max_value()`.

## Related Modules

- [`System`](https://docs.rs/frame-system/latest/frame_system/)
- [`Support`](https://docs.rs/frame-support/latest/frame_support/)

License: Apache-2.0

# Atomic Swap

A module for atomically sending funds.

- [`atomic_swap::Trait`](https://docs.rs/pallet-atomic-swap/latest/pallet_atomic_swap/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-atomic-swap/latest/pallet_atomic_swap/enum.Call.html)
- [`Module`](https://docs.rs/pallet-atomic-swap/latest/pallet_atomic_swap/struct.Module.html)

## Overview

A module for atomically sending funds from an origin to a target. A proof is used to allow the target to approve (claim) the swap. If the swap is not claimed within a specified duration of time, the sender may cancel it.

## Interface

### Dispatchable Functions

- `create_swap` - called by a sender to register a new atomic swap
- `claim_swap` - called by the target to approve a swap
- `cancel_swap` - may be called by a sender after a specified duration

License: Apache-2.0

# Aura Module

- [`aura::Trait`](https://docs.rs/pallet-aura/latest/pallet_aura/trait.Trait.html)
- [`Module`](https://docs.rs/pallet-aura/latest/pallet_aura/struct.Module.html)

## Overview

The Aura module extends Aura consensus by managing offline reporting.

## Interface

### Public Functions

- `slot_duration` - Determine the Aura slot-duration based on the Timestamp module configuration.

## Related Modules

- [Timestamp](https://docs.rs/pallet-timestamp/latest/pallet_timestamp/): The Timestamp module is used in Aura to track consensus rounds (via `slots`).

## References

If you're interested in hacking on this module, it is useful to understand the interaction with `substrate/primitives/inherents/src/lib.rs` and, specifically, the required implementation of [`ProvideInherent`](https://docs.rs/sp-inherents/latest/sp_inherents/trait.ProvideInherent.html) and [`ProvideInherentData`](https://docs.rs/sp-inherents/latest/sp_inherents/trait.ProvideInherentData.html) to create and check inherents.

License: Apache-2.0

# Authority discovery module.

This module is used by the `client/authority-discovery` to retrieve the current set of authorities.

License: Apache-2.0

# Authorship tracking for FRAME runtimes.

This tracks the current author of the block and recent uncles.

License: Apache-2.0

# Consensus extension module for BABE consensus. 

Collects on-chain randomness from VRF outputs and manages epoch transitions.

License: Apache-2.0



# Balances Module

The Balances module provides functionality for handling accounts and balances.

- [`balances::Trait`](https://docs.rs/pallet-balances/latest/pallet_balances/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-balances/latest/pallet_balances/enum.Call.html)
- [`Module`](https://docs.rs/pallet-balances/latest/pallet_balances/struct.Module.html)

## Overview

The Balances module provides functions for:

- Getting and setting free balances.
- Retrieving total, reserved and unreserved balances.
- Repatriating a reserved balance to a beneficiary account that exists.
- Transferring a balance between accounts (when not reserved).
- Slashing an account balance.
- Account creation and removal.
- Managing total issuance.
- Setting and managing locks.

### Terminology

- **Existential Deposit:** The minimum balance required to create or keep an account open. This prevents "dust accounts" from filling storage. When the free plus the reserved balance (i.e. the total balance) fall below this, then the account is said to be dead; and it loses its functionality as well as any prior history and all information on it is removed from the chain's state. No account should ever have a total balance that is strictly between 0 and the existential deposit (exclusive). If this ever happens, it indicates either a bug in this module or an erroneous raw mutation of storage.
- **Total Issuance:** The total number of units in existence in a system.
- **Reaping an account:** The act of removing an account by resetting its nonce. Happens after its total balance has become zero (or, strictly speaking, less than the Existential Deposit).
- **Free Balance:** The portion of a balance that is not reserved. The free balance is the only balance that matters for most operations.
- **Reserved Balance:** Reserved balance still belongs to the account holder, but is suspended. Reserved balance can still be slashed, but only after all the free balance has been slashed.
- **Imbalance:** A condition when some funds were credited or debited without equal and opposite accounting (i.e. a difference between total issuance and account balances). Functions that result in an imbalance will return an object of the `Imbalance` trait that can be managed within your runtime logic. (If an imbalance is simply dropped, it should automatically maintain any book-keeping such as total issuance.)
- **Lock:** A freeze on a specified amount of an account's free balance until a specified block number. Multiple locks always operate over the same funds, so they "overlay" rather than "stack".

### Implementations

The Balances module provides implementations for the following traits. If these traits provide the functionality that you need, then you can avoid coupling with the Balances module.

- [`Currency`](https://docs.rs/frame-support/latest/frame_support/traits/trait.Currency.html): Functions for dealing with a fungible assets system.
- [`ReservableCurrency`](https://docs.rs/frame-support/latest/frame_support/traits/trait.ReservableCurrency.html): Functions for dealing with assets that can be reserved from an account.
- [`LockableCurrency`](https://docs.rs/frame-support/latest/frame_support/traits/trait.LockableCurrency.html): Functions for dealing with accounts that allow liquidity restrictions.
- [`Imbalance`](https://docs.rs/frame-support/latest/frame_support/traits/trait.Imbalance.html): Functions for handling imbalances between total issuance in the system and account balances. Must be used when a function creates new funds (e.g. a reward) or destroys some funds (e.g. a system fee).
- [`IsDeadAccount`](https://docs.rs/frame-support/latest/frame_support/traits/trait.IsDeadAccount.html): Determiner to say whether a given account is unused.

## Interface

### Dispatchable Functions

- `transfer` - Transfer some liquid free balance to another account.
- `set_balance` - Set the balances of a given account. The origin of this call must be root.

## Usage

The following examples show how to use the Balances module in your custom module.

### Examples from the FRAME

The Contract module uses the `Currency` trait to handle gas payment, and its types inherit from `Currency`:

```rust
use frame_support::traits::Currency;

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;
```

The Staking module uses the `LockableCurrency` trait to lock a stash account's funds:

```rust
use frame_support::traits::{WithdrawReasons, LockableCurrency};
use sp_runtime::traits::Bounded;
pub trait Config: frame_system::Config {
	type Currency: LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;
}

fn update_ledger<T: Config>(
	controller: &T::AccountId,
	ledger: &StakingLedger<T>
) {
	T::Currency::set_lock(
		STAKING_ID,
		&ledger.stash,
		ledger.total,
		WithdrawReasons::all()
	);
	// <Ledger<T>>::insert(controller, ledger); // Commented out as we don't have access to Staking's storage here.
}
```

## Genesis config

The Balances module depends on the [`GenesisConfig`](https://docs.rs/pallet-balances/latest/pallet_balances/struct.GenesisConfig.html).

## Assumptions

- Total issued balanced of all accounts should be less than `Config::Balance::max_value()`.

License: Apache-2.0



# Substrate Runtime Benchmarking Framework

This crate contains a set of utilities that can be used to benchmark and weigh FRAME pallets that you develop for your Substrate Runtime.

## Overview

Substrate's FRAME framework allows you to develop custom logic for your blockchain that can be included in your runtime. This flexibility is key to help you design complex and interactive pallets, but without accurate weights assigned to dispatchables, your blockchain may become vulnerable to denial of service (DoS) attacks by malicious actors.

The Substrate Runtime Benchmarking Framework is a tool you can use to mitigate DoS attacks against your blockchain network by benchmarking the computational resources required to execute different functions in the runtime, for example extrinsics, `on_initialize`, `verify_unsigned`, etc...

The general philosophy behind the benchmarking system is: If your node can know ahead of time how long it will take to execute an extrinsic, it can safely make decisions to include or exclude that extrinsic based on its available resources. By doing this, it can keep the block production and import process running smoothly.

To achieve this, we need to model how long it takes to run each function in the runtime by:

- Creating custom benchmarking logic that executes a specific code path of a function.
- Executing the benchmark in the Wasm execution environment, on a specific set of hardware, with a custom runtime configuration, etc...
- Executing the benchmark across controlled ranges of possible values that may affect the result of the benchmark (called "components").
- Executing the benchmark multiple times at each point in order to isolate and remove outliers.
- Using the results of the benchmark to create a linear model of the function across its components.

With this linear model, we are able to estimate ahead of time how long it takes to execute some logic, and thus make informed decisions without actually spending any significant resources at runtime.

Note that we assume that all extrinsics are assumed to be of linear complexity, which is why we are able to always fit them to a linear model. Quadratic or higher complexity functions are, in general, considered to be dangerous to the runtime as the weight of these functions may explode as the runtime state or input becomes too complex.

The benchmarking framework comes with the following tools:

- [A set of macros](file:/Users/suyinrong/bitcoin-proj/substrate/frame/benchmarking/src/lib.rs) (`benchmarks!`, `add_benchmark!`, etc...) to make it easy to write, test, and add runtime benchmarks.
- [A set of linear regression analysis functions](file:/Users/suyinrong/bitcoin-proj/substrate/frame/benchmarking/src/analysis.rs) for processing benchmark data.
- [A CLI extension](file:/Users/suyinrong/bitcoin-proj/substrate/utils/frame/benchmarking-cli/) to make it easy to execute benchmarks on your node.

The end-to-end benchmarking pipeline is disabled by default when compiling a node. If you want to run benchmarks, you need to enable it by compiling with a Rust feature flag `runtime-benchmarks`. More details about this below.

### Weight

Substrate represents computational resources using a generic unit of measurement called "Weight". It defines 10^12 Weight as 1 second of computation on the physical machine used for benchmarking. This means that the weight of a function may change based on the specific hardware used to benchmark the runtime functions.

By modeling the expected weight of each runtime function, the blockchain is able to calculate how many transactions or system level functions it will be able to execute within a certain period of time. Often, the limiting factor for a blockchain is the fixed block production time for the network.

Within FRAME, each dispatchable function must have a `#[weight]` annotation with a function that can return the expected weight for the worst case scenario execution of that function given its inputs. This benchmarking framework will result in a file that automatically generates those formulas for you, which you can then use in your pallet.

## Writing Benchmarks

Writing a runtime benchmark is much like writing a unit test for your pallet. It needs to be carefully crafted to execute a certain logical path in your code. In tests you want to check for various success and failure conditions, but with benchmarks you specifically look for the **most computationally heavy** path, a.k.a the "worst case scenario".

This means that if there are certain storage items or runtime state that may affect the complexity of the function, for example triggering more iterations in a `for` loop, to get an accurate result, you must set up your benchmark to trigger this.

It may be that there are multiple paths your function can go down, and it is not clear which one is the heaviest. In this case, you should just create a benchmark for each scenario! You may find that there are paths in your code where complexity may become unbounded depending on user input. This may be a hint that you should enforce sane boundaries for how a user can use your pallet. For example: limiting the number of elements in a vector, limiting the number of iterations in a `for` loop, etc...

Examples of end-to-end benchmarks can be found in the [pallets provided by Substrate](file:/Users/suyinrong/bitcoin-proj/substrate/frame/), and the specific details on how to use the `benchmarks!` macro can be found in [its documentation](file:/Users/suyinrong/bitcoin-proj/substrate/frame/benchmarking/src/lib.rs).

## Testing Benchmarks

You can test your benchmarks using the same test runtime that you created for your pallet's unit tests. By creating your benchmarks in the `benchmarks!` macro, it automatically generates test functions for you:

```rust
fn test_benchmark_[benchmark_name]<T>::() -> Result<(), &'static str>
```

Simply add these functions to a unit test and ensure that the result of the function is `Ok(())`.

> **Note:** If your test runtime and production runtime have different configurations, you may get different results when testing your benchmark and actually running it.

In general, benchmarks returning `Ok(())` is all you need to check for since it signals the executed extrinsic has completed successfully. However, you can optionally include a `verify` block with your benchmark, which can additionally verify any final conditions, such as the final state of your runtime.

These additional `verify` blocks will not affect the results of your final benchmarking process.

To run the tests, you need to enable the `runtime-benchmarks` feature flag. This may also mean you need to move into your node's binary folder. For example, with the Substrate repository, this is how you would test the Balances pallet's benchmarks:

```bash
cargo test -p pallet-balances --features runtime-benchmarks
```

> NOTE: Substrate uses a virtual workspace which does not allow you to compile with feature flags.
>
> ```
> error: --features is not allowed in the root of a virtual workspace`
> ```
>
> To solve this, navigate to the folder of the node (`cd bin/node/cli`) or pallet (`cd frame/pallet`) and run the command there.

## Adding Benchmarks

The benchmarks included with each pallet are not automatically added to your node. To actually execute these benchmarks, you need to implement the `frame_benchmarking::Benchmark` trait. You can see an example of how to do this in the [included Substrate node](file:/Users/suyinrong/bitcoin-proj/substrate/bin/node/runtime/src/lib.rs).

Assuming there are already some benchmarks set up on your node, you just need to add another instance of the `add_benchmark!` macro:

```rust
///  configuration for running benchmarks
///               |    name of your pallet's crate (as imported)
///               v                   v
add_benchmark!(params, batches, pallet_balances, Balances);
///                       ^                          ^
///    where all benchmark results are saved         |
///            the `struct` created for your pallet by `construct_runtime!`
```

Once you have done this, you will need to compile your node binary with the `runtime-benchmarks` feature flag:

```bash
cd bin/node/cli
cargo build --release --features runtime-benchmarks
```

## Running Benchmarks

Finally, once you have a node binary with benchmarks enabled, you need to execute your various benchmarks.

You can get a list of the available benchmarks by running:

```bash
./target/release/substrate benchmark --chain dev --pallet "*" --extrinsic "*" --repeat 0
```

Then you can run a benchmark like so:

```bash
./target/release/substrate benchmark \
    --chain dev \                  # Configurable Chain Spec
    --execution=wasm \             # Always test with Wasm
    --wasm-execution=compiled \    # Always used `wasm-time`
    --pallet pallet_balances \     # Select the pallet
    --extrinsic transfer \         # Select the extrinsic
    --steps 50 \                   # Number of samples across component ranges
    --repeat 20 \                  # Number of times we repeat a benchmark
    --output <path> \              # Output benchmark results into a folder or file
```

This will output a file `pallet_name.rs` which implements the `WeightInfo` trait you should include in your pallet. Each blockchain should generate their own benchmark file with their custom implementation of the `WeightInfo` trait. This means that you will be able to use these modular Substrate pallets while still keeping your network safe for your specific configuration and requirements.

The benchmarking CLI uses a Handlebars template to format the final output file. You can optionally pass the flag `--template` pointing to a custom template that can be used instead. Within the template, you have access to all the data provided by the `TemplateData` struct in the [benchmarking CLI writer](file:/Users/suyinrong/bitcoin-proj/substrate/utils/frame/benchmarking-cli/src/writer.rs). You can find the default template used [here](file:/Users/suyinrong/bitcoin-proj/substrate/utils/frame/benchmarking-cli/src/template.hbs).

There are some custom Handlebars helpers included with our output generation:

- `underscore`: Add an underscore to every 3rd character from the right of a string. Primarily to be used for delimiting large numbers.
- `join`: Join an array of strings into a space-separated string for the template. Primarily to be used for joining all the arguments passed to the CLI.

To get a full list of available options when running benchmarks, run:

```bash
./target/release/substrate benchmark --help
```

License: Apache-2.0



# Bounties Module ( pallet-bounties )

## Bounty

**Note :: This pallet is tightly coupled with pallet-treasury**

A Bounty Spending is a reward for a specified body of work - or specified set of objectives - that needs to be executed for a predefined Treasury amount to be paid out. A curator is assigned after the bounty is approved and funded by Council, to be delegated with the responsibility of assigning a payout address once the specified set of objectives is completed.

After the Council has activated a bounty, it delegates the work that requires expertise to a curator in exchange of a deposit. Once the curator accepts the bounty, they get to close the active bounty. Closing the active bounty enacts a delayed payout to the payout address, the curator fee and the return of the curator deposit. The delay allows for intervention through regular democracy. The Council gets to unassign the curator, resulting in a new curator election. The Council also gets to cancel the bounty if deemed necessary before assigning a curator or once the bounty is active or payout is pending, resulting in the slash of the curator's deposit.

### Terminology

- **Bounty spending proposal:** A proposal to reward a predefined body of work upon completion by the Treasury.
- **Proposer:** An account proposing a bounty spending.
- **Curator:** An account managing the bounty and assigning a payout address receiving the reward for the completion of work.
- **Deposit:** The amount held on deposit for placing a bounty proposal plus the amount held on deposit per byte within the bounty description.
- **Curator deposit:** The payment from a candidate willing to curate an approved bounty. The deposit is returned when/if the bounty is completed.
- **Bounty value:** The total amount that should be paid to the Payout Address if the bounty is rewarded.
- **Payout address:** The account to which the total or part of the bounty is assigned to.
- **Payout Delay:** The delay period for which a bounty beneficiary needs to wait before claiming.
- **Curator fee:** The reserved upfront payment for a curator for work related to the bounty.

## Interface

### Dispatchable Functions

Bounty protocol:

- `propose_bounty` - Propose a specific treasury amount to be earmarked for a predefined set of tasks and stake the required deposit.
- `approve_bounty` - Accept a specific treasury amount to be earmarked for a predefined body of work.
- `propose_curator` - Assign an account to a bounty as candidate curator.
- `accept_curator` - Accept a bounty assignment from the Council, setting a curator deposit.
- `extend_bounty_expiry` - Extend the expiry block number of the bounty and stay active.
- `award_bounty` - Close and pay out the specified amount for the completed work.
- `claim_bounty` - Claim a specific bounty amount from the Payout Address.
- `unassign_curator` - Unassign an accepted curator from a specific earmark.
- `close_bounty` - Cancel the earmark for a specific treasury amount and close the bounty.

# Collective

Collective system: Members of a set of account IDs can make their collective feelings known through dispatched calls from one of two specialized origins.

The membership can be provided in one of two ways: either directly, using the Root-dispatchable function `set_members`, or indirectly, through implementing the `ChangeMembers`. The pallet assumes that the amount of members stays at or below `MaxMembers` for its weight calculations, but enforces this neither in `set_members` nor in `change_members_sorted`.

A "prime" member may be set to help determine the default vote behavior based on chain config. If `PreimDefaultVote` is used, the prime vote acts as the default vote in case of any abstentions after the voting period. If `MoreThanMajorityThenPrimeDefaultVote` is used, then abstentations will first follow the majority of the collective voting, and then the prime member.

Voting happens through motions comprising a proposal (i.e. a curried dispatchable) plus a number of approvals required for it to pass and be called. Motions are open for members to vote on for a minimum period given by `MotionDuration`. As soon as the needed number of approvals is given, the motion is closed and executed. If the number of approvals is not reached during the voting period, then `close` may be called by any account in order to force the end the motion explicitly. If a prime member is defined then their vote is used in place of any abstentions and the proposal is executed if there are enough approvals counting the new votes.

If there are not, or if no prime is set, then the motion is dropped without being executed.

License: Apache-2.0



# Contract Module

The Contract module provides functionality for the runtime to deploy and execute WebAssembly smart-contracts.

- [`contract::Trait`](https://docs.rs/pallet-contracts/latest/pallet_contracts/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-contracts/latest/pallet_contracts/enum.Call.html)

## Overview

This module extends accounts based on the `Currency` trait to have smart-contract functionality. It can be used with other modules that implement accounts based on `Currency`. These "smart-contract accounts" have the ability to instantiate smart-contracts and make calls to other contract and non-contract accounts.

The smart-contract code is stored once in a `code_cache`, and later retrievable via its `code_hash`. This means that multiple smart-contracts can be instantiated from the same `code_cache`, without replicating the code each time.

When a smart-contract is called, its associated code is retrieved via the code hash and gets executed. This call can alter the storage entries of the smart-contract account, instantiate new smart-contracts, or call other smart-contracts.

Finally, when an account is reaped, its associated code and storage of the smart-contract account will also be deleted.

### Gas

Senders must specify a gas limit with every call, as all instructions invoked by the smart-contract require gas. Unused gas is refunded after the call, regardless of the execution outcome.

If the gas limit is reached, then all calls and state changes (including balance transfers) are only reverted at the current call's contract level. For example, if contract A calls B and B runs out of gas mid-call, then all of B's calls are reverted. Assuming correct error handling by contract A, A's other calls and state changes still persist.

### Notable Scenarios

Contract call failures are not always cascading. When failures occur in a sub-call, they do not "bubble up", and the call will only revert at the specific contract level. For example, if contract A calls contract B, and B fails, A can decide how to handle that failure, either proceeding or reverting A's changes.

## Interface

### Dispatchable functions

- `put_code` - Stores the given binary Wasm code into the chain's storage and returns its `code_hash`.
- `instantiate` - Deploys a new contract from the given `code_hash`, optionally transferring some balance. This instantiates a new smart contract account and calls its contract deploy handler to initialize the contract.
- `call` - Makes a call to an account, optionally transferring some balance.

## Usage

The Contract module is a work in progress. The following examples show how this Contract module can be used to instantiate and call contracts.

- [`ink`](https://github.com/paritytech/ink) is an [`eDSL`](https://wiki.haskell.org/Embedded_domain_specific_language) that enables writing WebAssembly based smart contracts in the Rust programming language. This is a work in progress.

## Related Modules

- [Balances](https://docs.rs/pallet-balances/latest/pallet_balances/)

License: Apache-2.0



# Democracy Pallet

- [`democracy::Trait`](https://docs.rs/pallet-democracy/latest/pallet_democracy/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-democracy/latest/pallet_democracy/enum.Call.html)

## Overview

The Democracy pallet handles the administration of general stakeholder voting.

There are two different queues that a proposal can be added to before it becomes a referendum, 1) the proposal queue consisting of all public proposals and 2) the external queue consisting of a single proposal that originates from one of the *external* origins (such as a collective group).

Every launch period - a length defined in the runtime - the Democracy pallet launches a referendum from a proposal that it takes from either the proposal queue or the external queue in turn. Any token holder in the system can vote on referenda. The voting system uses time-lock voting by allowing the token holder to set their *conviction* behind a vote. The conviction will dictate the length of time the tokens will be locked, as well as the multiplier that scales the vote power.

### Terminology

- **Enactment Period:** The minimum period of locking and the period between a proposal being approved and enacted.
- **Lock Period:** A period of time after proposal enactment that the tokens of *winning* voters will be locked.
- **Conviction:** An indication of a voter's strength of belief in their vote. An increase of one in conviction indicates that a token holder is willing to lock their tokens for twice as many lock periods after enactment.
- **Vote:** A value that can either be in approval ("Aye") or rejection ("Nay") of a particular referendum.
- **Proposal:** A submission to the chain that represents an action that a proposer (either an account or an external origin) suggests that the system adopt.
- **Referendum:** A proposal that is in the process of being voted on for either acceptance or rejection as a change to the system.
- **Delegation:** The act of granting your voting power to the decisions of another account for up to a certain conviction.

### Adaptive Quorum Biasing

A *referendum* can be either simple majority-carries in which 50%+1 of the votes decide the outcome or *adaptive quorum biased*. Adaptive quorum biasing makes the threshold for passing or rejecting a referendum higher or lower depending on how the referendum was originally proposed. There are two types of adaptive quorum biasing: 1) *positive turnout bias* makes a referendum require a super-majority to pass that decreases as turnout increases and

1. *negative turnout bias* makes a referendum require a super-majority to reject that decreases as turnout increases. Another way to think about the quorum biasing is that *positive bias* referendums will be rejected by default and *negative bias* referendums get passed by default.

## Interface

### Dispatchable Functions

#### Public

These calls can be made from any externally held account capable of creating a signed extrinsic.

Basic actions:

- `propose` - Submits a sensitive action, represented as a hash. Requires a deposit.
- `second` - Signals agreement with a proposal, moves it higher on the proposal queue, and requires a matching deposit to the original.
- `vote` - Votes in a referendum, either the vote is "Aye" to enact the proposal or "Nay" to keep the status quo.
- `unvote` - Cancel a previous vote, this must be done by the voter before the vote ends.
- `delegate` - Delegates the voting power (tokens * conviction) to another account.
- `undelegate` - Stops the delegation of voting power to another account.

Administration actions that can be done to any account:

- `reap_vote` - Remove some account's expired votes.
- `unlock` - Redetermine the account's balance lock, potentially making tokens available.

Preimage actions:

- `note_preimage` - Registers the preimage for an upcoming proposal, requires a deposit that is returned once the proposal is enacted.
- `note_preimage_operational` - same but provided by `T::OperationalPreimageOrigin`.
- `note_imminent_preimage` - Registers the preimage for an upcoming proposal. Does not require a deposit, but the proposal must be in the dispatch queue.
- `note_imminent_preimage_operational` - same but provided by `T::OperationalPreimageOrigin`.
- `reap_preimage` - Removes the preimage for an expired proposal. Will only work under the condition that it's the same account that noted it and after the voting period, OR it's a different account after the enactment period.

#### Cancellation Origin

This call can only be made by the `CancellationOrigin`.

- `emergency_cancel` - Schedules an emergency cancellation of a referendum. Can only happen once to a specific referendum.

#### ExternalOrigin

This call can only be made by the `ExternalOrigin`.

- `external_propose` - Schedules a proposal to become a referendum once it is is legal for an externally proposed referendum.

#### External Majority Origin

This call can only be made by the `ExternalMajorityOrigin`.

- `external_propose_majority` - Schedules a proposal to become a majority-carries referendum once it is legal for an externally proposed referendum.

#### External Default Origin

This call can only be made by the `ExternalDefaultOrigin`.

- `external_propose_default` - Schedules a proposal to become a negative-turnout-bias referendum once it is legal for an externally proposed referendum.

#### Fast Track Origin

This call can only be made by the `FastTrackOrigin`.

- `fast_track` - Schedules the current externally proposed proposal that is "majority-carries" to become a referendum immediately.

#### Veto Origin

This call can only be made by the `VetoOrigin`.

- `veto_external` - Vetoes and blacklists the external proposal hash.

#### Root

- `cancel_referendum` - Removes a referendum.
- `cancel_queued` - Cancels a proposal that is queued for enactment.
- `clear_public_proposal` - Removes all public proposals.

License: Apache-2.0

# Elections



Election module for stake-weighted membership selection of a collective.

The composition of a set of account IDs works according to one or more approval votes weighted by stake. There is a partial carry-over facility to give greater weight to those whose voting is serially unsuccessful.

License: Apache-2.0



# Phragmén Election Module.

An election module based on sequential phragmen.

### Term and Round

The election happens in *rounds*: every `N` blocks, all previous members are retired and a new set is elected (which may or may not have an intersection with the previous set). Each round lasts for some number of blocks defined by `TermDuration` storage item. The words *term* and *round* can be used interchangeably in this context.

`TermDuration` might change during a round. This can shorten or extend the length of the round. The next election round's block number is never stored but rather always checked on the fly. Based on the current block number and `TermDuration`, the condition `BlockNumber % TermDuration == 0` being satisfied will always trigger a new election round.

### Voting

Voters can vote for any set of the candidates by providing a list of account ids. Invalid votes (voting for non-candidates) are ignored during election. Yet, a voter *might* vote for a future candidate. Voters reserve a bond as they vote. Each vote defines a `value`. This amount is locked from the account of the voter and indicates the weight of the vote. Voters can update their votes at any time by calling `vote()` again. This keeps the bond untouched but can optionally change the locked `value`. After a round, votes are kept and might still be valid for further rounds. A voter is responsible for calling `remove_voter` once they are done to have their bond back and remove the lock.

Voters also report other voters as being defunct to earn their bond. A voter is defunct once all of the candidates that they have voted for are neither a valid candidate anymore nor a member. Upon reporting, if the target voter is actually defunct, the reporter will be rewarded by the voting bond of the target. The target will lose their bond and get removed. If the target is not defunct, the reporter is slashed and removed. To prevent being reported, voters should manually submit a `remove_voter()` as soon as they are in the defunct state.

### Candidacy and Members

Candidates also reserve a bond as they submit candidacy. A candidate cannot take their candidacy back. A candidate can end up in one of the below situations:

- **Winner**: A winner is kept as a *member*. They must still have a bond in reserve and they are automatically counted as a candidate for the next election.
- **Runner-up**: Runners-up are the best candidates immediately after the winners. The number of runners_up to keep is configurable. Runners-up are used, in order that they are elected, as replacements when a candidate is kicked by `[remove_member]`, or when an active member renounces their candidacy. Runners are automatically counted as a candidate for the next election.
- **Loser**: Any of the candidate who are not a winner are left as losers. A loser might be an *outgoing member or runner*, meaning that they are an active member who failed to keep their spot. An outgoing will always lose their bond.

##### Renouncing candidacy.

All candidates, elected or not, can renounce their candidacy. A call to [`Module::renounce_candidacy`] will always cause the candidacy bond to be refunded.

Note that with the members being the default candidates for the next round and votes persisting in storage, the election system is entirely stable given no further input. This means that if the system has a particular set of candidates `C` and voters `V` that lead to a set of members `M` being elected, as long as `V` and `C` don't remove their candidacy and votes, `M` will keep being re-elected at the end of each round.

### Module Information

- [`election_sp_phragmen::Trait`](https://docs.rs/pallet-elections-phragmen/latest/pallet_elections_phragmen/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-elections-phragmen/latest/pallet_elections_phragmen/enum.Call.html)
- [`Module`](https://docs.rs/pallet-elections-phragmen/latest/pallet_elections_phragmen/struct.Module.html)

License: Apache-2.0



# Example Pallet

The Example: A simple example of a FRAME pallet demonstrating concepts, APIs and structures common to most FRAME runtimes.

Run `cargo doc --package pallet-example --open` to view this pallet's documentation.

### Documentation Guidelines:

- Documentation comments (i.e. `/// comment`) - should accompany pallet functions and be restricted to the pallet interface, not the internals of the pallet implementation. Only state inputs, outputs, and a brief description that mentions whether calling it requires root, but without repeating the source code details. Capitalize the first word of each documentation comment and end it with a full stop. See [Generic example of annotating source code with documentation comments](https://github.com/paritytech/substrate#72-contributing-to-documentation-for-substrate-packages)
- Self-documenting code - Try to refactor code to be self-documenting.
- Code comments - Supplement complex code with a brief explanation, not every line of code.
- Identifiers - surround by backticks (i.e. `INHERENT_IDENTIFIER`, `InherentType`, `u64`)
- Usage scenarios - should be simple doctests. The compiler should ensure they stay valid.
- Extended tutorials - should be moved to external files and refer to.
- Mandatory - include all of the sections/subsections where **MUST** is specified.
- Optional - optionally include sections/subsections where **CAN** is specified.

### Documentation Template: 

Copy and paste this template from frame/example/src/lib.rs into file `frame/<INSERT_CUSTOM_PALLET_NAME>/src/lib.rs` of your own custom pallet and complete it.

<details style="box-sizing: border-box; color: rgb(187, 187, 187); font-family: Helvetica, Arial, freesans, sans-serif; font-size: 14px; font-style: normal; font-variant-ligatures: normal; font-variant-caps: normal; font-weight: 400; letter-spacing: normal; orphans: 2; text-align: start; text-indent: 0px; text-transform: none; white-space: normal; widows: 2; word-spacing: 0px; -webkit-text-stroke-width: 0px; background-color: rgb(43, 43, 43); text-decoration-style: initial; text-decoration-color: initial;"></details>

License: Unlicense



# Offchain Worker Example Module

The Offchain Worker Example: A simple pallet demonstrating concepts, APIs and structures common to most offchain workers.

Run `cargo doc --package pallet-example-offchain-worker --open` to view this module's documentation.

- [`pallet_example_offchain_worker::Trait`](file:/Users/suyinrong/bitcoin-proj/substrate/frame/example-offchain-worker/trait.Trait.html)
- [`Call`](file:/Users/suyinrong/bitcoin-proj/substrate/frame/example-offchain-worker/enum.Call.html)
- [`Module`](file:/Users/suyinrong/bitcoin-proj/substrate/frame/example-offchain-worker/struct.Module.html)

## Overview

In this example we are going to build a very simplistic, naive and definitely NOT production-ready oracle for BTC/USD price. Offchain Worker (OCW) will be triggered after every block, fetch the current price and prepare either signed or unsigned transaction to feed the result back on chain. The on-chain logic will simply aggregate the results and store last `64` values to compute the average price. Additional logic in OCW is put in place to prevent spamming the network with both signed and unsigned transactions, and custom `UnsignedValidator` makes sure that there is only one unsigned transaction floating in the network.

License: Unlicense



# Executive Module

The Executive module acts as the orchestration layer for the runtime. It dispatches incoming extrinsic calls to the respective modules in the runtime.

## Overview

The executive module is not a typical pallet providing functionality around a specific feature. It is a cross-cutting framework component for the FRAME. It works in conjunction with the [FRAME System module](https://docs.rs/frame-system/latest/frame_system/) to perform these cross-cutting functions.

The Executive module provides functions to:

- Check transaction validity.
- Initialize a block.
- Apply extrinsics.
- Execute a block.
- Finalize a block.
- Start an off-chain worker.

### Implementations

The Executive module provides the following implementations:

- `ExecuteBlock`: Trait that can be used to execute a block.
- `Executive`: Type that can be used to make the FRAME available from the runtime.

## Usage

The default Substrate node template declares the [`Executive`](https://docs.rs/frame-executive/latest/frame_executive/struct.Executive.html) type in its library.

### Example

`Executive` type declaration from the node template.

```rust
#
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Runtime, AllModules>;
```

### Custom `OnRuntimeUpgrade` logic

You can add custom logic that should be called in your runtime on a runtime upgrade. This is done by setting an optional generic parameter. The custom logic will be called before the on runtime upgrade logic of all modules is called.

```rust
#
struct CustomOnRuntimeUpgrade;
impl frame_support::traits::OnRuntimeUpgrade for CustomOnRuntimeUpgrade {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        // Do whatever you want.
        0
    }
}

pub type Executive = executive::Executive<Runtime, Block, Context, Runtime, AllModules, CustomOnRuntimeUpgrade>;
```

License: Apache-2.0



# GRANDPA



GRANDPA Consensus module for runtime.

This manages the GRANDPA authority set ready for the native code. These authorities are only for GRANDPA finality, not for consensus overall.

In the future, it will also handle misbehavior reports, and on-chain finality notifications.

For full integration with GRANDPA, the `GrandpaApi` should be implemented. The necessary items are re-exported via the `fg_primitives` crate.

License: Apache-2.0



# Identity Module

- [`identity::Trait`](https://docs.rs/pallet-identity/latest/pallet_identity/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-identity/latest/pallet_identity/enum.Call.html)

## Overview

A federated naming system, allowing for multiple registrars to be added from a specified origin. Registrars can set a fee to provide identity-verification service. Anyone can put forth a proposed identity for a fixed deposit and ask for review by any number of registrars (paying each of their fees). Registrar judgements are given as an `enum`, allowing for sophisticated, multi-tier opinions.

Some judgements are identified as *sticky*, which means they cannot be removed except by complete removal of the identity, or by the registrar. Judgements are allowed to represent a portion of funds that have been reserved for the registrar.

A super-user can remove accounts and in doing so, slash the deposit.

All accounts may also have a limited number of sub-accounts which may be specified by the owner; by definition, these have equivalent ownership and each has an individual name.

The number of registrars should be limited, and the deposit made sufficiently large, to ensure no state-bloat attack is viable.

## Interface

### Dispatchable Functions

#### For general users

- `set_identity` - Set the associated identity of an account; a small deposit is reserved if not already taken.
- `clear_identity` - Remove an account's associated identity; the deposit is returned.
- `request_judgement` - Request a judgement from a registrar, paying a fee.
- `cancel_request` - Cancel the previous request for a judgement.

#### For general users with sub-identities

- `set_subs` - Set the sub-accounts of an identity.
- `add_sub` - Add a sub-identity to an identity.
- `remove_sub` - Remove a sub-identity of an identity.
- `rename_sub` - Rename a sub-identity of an identity.
- `quit_sub` - Remove a sub-identity of an identity (called by the sub-identity).

#### For registrars

- `set_fee` - Set the fee required to be paid for a judgement to be given by the registrar.
- `set_fields` - Set the fields that a registrar cares about in their judgements.
- `provide_judgement` - Provide a judgement to an identity.

#### For super-users

- `add_registrar` - Add a new registrar to the system.
- `kill_identity` - Forcibly remove the associated identity; the deposit is lost.

License: Apache-2.0

# I'm online Module

If the local node is a validator (i.e. contains an authority key), this module gossips a heartbeat transaction with each new session. The heartbeat functions as a simple mechanism to signal that the node is online in the current era.

Received heartbeats are tracked for one era and reset with each new era. The module exposes two public functions to query if a heartbeat has been received in the current era or session.

The heartbeat is a signed transaction, which was signed using the session key and includes the recent best block number of the local validators chain as well as the `NetworkState`. It is submitted as an Unsigned Transaction via off-chain workers.

- [`im_online::Trait`](https://docs.rs/pallet-im-online/latest/pallet_im_online/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-im-online/latest/pallet_im_online/enum.Call.html)
- [`Module`](https://docs.rs/pallet-im-online/latest/pallet_im_online/struct.Module.html)

## Interface

### Public Functions

- `is_online` - True if the validator sent a heartbeat in the current session.

## Usage

```rust
use frame_support::{decl_module, dispatch};
use frame_system::ensure_signed;
use pallet_im_online::{self as im_online};

pub trait Config: im_online::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		pub fn is_online(origin, authority_index: u32) -> dispatch::DispatchResult {
			let _sender = ensure_signed(origin)?;
			let _is_online = <im_online::Module<T>>::is_online(authority_index);
			Ok(())
		}
	}
}
```

## Dependencies

This module depends on the [Session module](https://docs.rs/pallet-session/latest/pallet_session/).

License: Apache-2.0

# Indices

An index is a short form of an address. This module handles allocation of indices for a newly created accounts.

License: Apache-2.0

# Membership Module

Allows control of membership of a set of `AccountId`s, useful for managing membership of of a collective. A prime member may be set.

License: Apache-2.0

# Metadata

Decodable variant of the RuntimeMetadata.

This really doesn't belong here, but is necessary for the moment. In the future it should be removed entirely to an external module for shimming on to the codec-encoded metadata.

License: Apache-2.0



# Multisig Module

A module for doing multisig dispatch.

- [`multisig::Trait`](https://docs.rs/pallet-multisig/latest/pallet_multisig/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-multisig/latest/pallet_multisig/enum.Call.html)

## Overview

This module contains functionality for multi-signature dispatch, a (potentially) stateful operation, allowing multiple signed origins (accounts) to coordinate and dispatch a call from a well-known origin, derivable deterministically from the set of account IDs and the threshold number of accounts from the set that must approve it. In the case that the threshold is just one then this is a stateless operation. This is useful for multisig wallets where cryptographic threshold signatures are not available or desired.

## Interface

### Dispatchable Functions

- `as_multi` - Approve and if possible dispatch a call from a composite origin formed from a number of signed origins.
- `approve_as_multi` - Approve a call from a composite origin.
- `cancel_as_multi` - Cancel a call from a composite origin.

License: Apache-2.0

# Nicks Module

- [`nicks::Trait`](https://docs.rs/pallet-nicks/latest/pallet_nicks/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-nicks/latest/pallet_nicks/enum.Call.html)

## Overview

Nicks is an example module for keeping track of account names on-chain. It makes no effort to create a name hierarchy, be a DNS replacement or provide reverse lookups. Furthermore, the weights attached to this module's dispatchable functions are for demonstration purposes only and have not been designed to be economically secure. Do not use this pallet as-is in production.

## Interface

### Dispatchable Functions

- `set_name` - Set the associated name of an account; a small deposit is reserved if not already taken.
- `clear_name` - Remove an account's associated name; the deposit is returned.
- `kill_name` - Forcibly remove the associated name; the deposit is lost.

License: Apache-2.0

# Offences Module

Tracks reported offences

License: Apache-2.0

# Proxy Module

A module allowing accounts to give permission to other accounts to dispatch types of calls from their signed origin.

The accounts to which permission is delegated may be requied to announce the action that they wish to execute some duration prior to execution happens. In this case, the target account may reject the announcement and in doing so, veto the execution.

- [`proxy::Trait`](https://docs.rs/pallet-proxy/latest/pallet_proxy/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-proxy/latest/pallet_proxy/enum.Call.html)

## Overview

## Interface

### Dispatchable Functions

License: Apache-2.0

# Randomness Module

The Randomness Collective Flip module provides a [`random`](https://docs.rs/pallet-randomness-collective-flip/latest/pallet_randomness_collective_flip/struct.Module.html#method.random) function that generates low-influence random values based on the block hashes from the previous `81` blocks. Low-influence randomness can be useful when defending against relatively weak adversaries. Using this pallet as a randomness source is advisable primarily in low-security situations like testing.

## Public Functions

See the [`Module`](https://docs.rs/pallet-randomness-collective-flip/latest/pallet_randomness_collective_flip/struct.Module.html) struct for details of publicly available functions.

## Usage

### Prerequisites

Import the Randomness Collective Flip module and derive your module's configuration trait from the system trait.

### Example - Get random seed for the current block

```rust
use frame_support::{decl_module, dispatch, traits::Randomness};

pub trait Config: frame_system::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		pub fn random_module_example(origin) -> dispatch::DispatchResult {
			let _random_value = <pallet_randomness_collective_flip::Module<T>>::random(&b"my context"[..]);
			Ok(())
		}
	}
}
```

License: Apache-2.0

# Recovery Pallet

- [`recovery::Trait`](https://docs.rs/pallet-recovery/latest/pallet_recovery/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-recovery/latest/pallet_recovery/enum.Call.html)

## Overview

The Recovery pallet is an M-of-N social recovery tool for users to gain access to their accounts if the private key or other authentication mechanism is lost. Through this pallet, a user is able to make calls on-behalf-of another account which they have recovered. The recovery process is protected by trusted "friends" whom the original account owner chooses. A threshold (M) out of N friends are needed to give another account access to the recoverable account.

### Recovery Configuration

The recovery process for each recoverable account can be configured by the account owner. They are able to choose:

- `friends` - The list of friends that the account owner trusts to protect the recovery process for their account.
- `threshold` - The number of friends that need to approve a recovery process for the account to be successfully recovered.
- `delay_period` - The minimum number of blocks after the beginning of the recovery process that need to pass before the account can be successfully recovered.

There is a configurable deposit that all users need to pay to create a recovery configuration. This deposit is composed of a base deposit plus a multiplier for the number of friends chosen. This deposit is returned in full when the account owner removes their recovery configuration.

### Recovery Life Cycle

The intended life cycle of a successful recovery takes the following steps:

1. The account owner calls `create_recovery` to set up a recovery configuration for their account.
2. At some later time, the account owner loses access to their account and wants to recover it. Likely, they will need to create a new account and fund it with enough balance to support the transaction fees and the deposit for the recovery process.
3. Using this new account, they call `initiate_recovery`.
4. Then the account owner would contact their configured friends to vouch for the recovery attempt. The account owner would provide their old account id and the new account id, and friends would call `vouch_recovery` with those parameters.
5. Once a threshold number of friends have vouched for the recovery attempt, the account owner needs to wait until the delay period has passed, starting when they initiated the recovery process.
6. Now the account owner is able to call `claim_recovery`, which subsequently allows them to call `as_recovered` and directly make calls on-behalf-of the lost account.
7. Using the now recovered account, the account owner can call `close_recovery` on the recovery process they opened, reclaiming the recovery deposit they placed.
8. Then the account owner should then call `remove_recovery` to remove the recovery configuration on the recovered account and reclaim the recovery configuration deposit they placed.
9. Using `as_recovered`, the account owner is able to call any other pallets to clean up their state and reclaim any reserved or locked funds. They can then transfer all funds from the recovered account to the new account.
10. When the recovered account becomes reaped (i.e. its free and reserved balance drops to zero), the final recovery link is removed.

### Malicious Recovery Attempts

Initializing a the recovery process for a recoverable account is open and permissionless. However, the recovery deposit is an economic deterrent that should disincentivize would-be attackers from trying to maliciously recover accounts.

The recovery deposit can always be claimed by the account which is trying to to be recovered. In the case of a malicious recovery attempt, the account owner who still has access to their account can claim the deposit and essentially punish the malicious user.

Furthermore, the malicious recovery attempt can only be successful if the attacker is also able to get enough friends to vouch for the recovery attempt. In the case where the account owner prevents a malicious recovery process, this pallet makes it near-zero cost to re-configure the recovery settings and remove/replace friends who are acting inappropriately.

### Safety Considerations

It is important to note that this is a powerful pallet that can compromise the security of an account if used incorrectly. Some recommended practices for users of this pallet are:

- Configure a significant `delay_period` for your recovery process: As long as you have access to your recoverable account, you need only check the blockchain once every `delay_period` blocks to ensure that no recovery attempt is successful against your account. Using off-chain notification systems can help with this, but ultimately, setting a large `delay_period` means that even the most skilled attacker will need to wait this long before they can access your account.
- Use a high threshold of approvals: Setting a value of 1 for the threshold means that any of your friends would be able to recover your account. They would simply need to start a recovery process and approve their own process. Similarly, a threshold of 2 would mean that any 2 friends could work together to gain access to your account. The only way to prevent against these kinds of attacks is to choose a high threshold of approvals and select from a diverse friend group that would not be able to reasonably coordinate with one another.
- Reset your configuration over time: Since the entire deposit of creating a recovery configuration is returned to the user, the only cost of updating your recovery configuration is the transaction fees for the calls. Thus, it is strongly encouraged to regularly update your recovery configuration as your life changes and your relationship with new and existing friends change as well.

## Interface

### Dispatchable Functions

#### For General Users

- `create_recovery` - Create a recovery configuration for your account and make it recoverable.
- `initiate_recovery` - Start the recovery process for a recoverable account.

#### For Friends of a Recoverable Account

- `vouch_recovery` - As a `friend` of a recoverable account, vouch for a recovery attempt on the account.

#### For a User Who Successfully Recovered an Account

- `claim_recovery` - Claim access to the account that you have successfully completed the recovery process for.
- `as_recovered` - Send a transaction as an account that you have recovered. See other functions below.

#### For the Recoverable Account

- `close_recovery` - Close an active recovery process for your account and reclaim the recovery deposit.
- `remove_recovery` - Remove the recovery configuration from the account, making it un-recoverable.

#### For Super Users

- `set_recovered` - The ROOT origin is able to skip the recovery process and directly allow one account to access another.

License: Apache-2.0

# Scheduler

A module for scheduling dispatches.

- [`scheduler::Trait`](https://docs.rs/pallet-scheduler/latest/pallet_scheduler/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-scheduler/latest/pallet_scheduler/enum.Call.html)
- [`Module`](https://docs.rs/pallet-scheduler/latest/pallet_scheduler/struct.Module.html)

## Overview

This module exposes capabilities for scheduling dispatches to occur at a specified block number or at a specified period. These scheduled dispatches may be named or anonymous and may be canceled.

**NOTE:** The scheduled calls will be dispatched with the default filter for the origin: namely `frame_system::Config::BaseCallFilter` for all origin except root which will get no filter. And not the filter contained in origin use to call `fn schedule`.

If a call is scheduled using proxy or whatever mecanism which adds filter, then those filter will not be used when dispatching the schedule call.

## Interface

### Dispatchable Functions

- `schedule` - schedule a dispatch, which may be periodic, to occur at a specified block and with a specified priority.
- `cancel` - cancel a scheduled dispatch, specified by block number and index.
- `schedule_named` - augments the `schedule` interface with an additional `Vec<u8>` parameter that can be used for identification.
- `cancel_named` - the named complement to the cancel function.

License: Unlicense

# Scored Pool Module

The module maintains a scored membership pool. Each entity in the pool can be attributed a `Score`. From this pool a set `Members` is constructed. This set contains the `MemberCount` highest scoring entities. Unscored entities are never part of `Members`.

If an entity wants to be part of the pool a deposit is required. The deposit is returned when the entity withdraws or when it is removed by an entity with the appropriate authority.

Every `Period` blocks the set of `Members` is refreshed from the highest scoring members in the pool and, no matter if changes occurred, `T::MembershipChanged::set_members_sorted` is invoked. On first load `T::MembershipInitialized::initialize_members` is invoked with the initial `Members` set.

It is possible to withdraw candidacy/resign your membership at any time. If an entity is currently a member, this results in removal from the `Pool` and `Members`; the entity is immediately replaced by the next highest scoring candidate in the pool, if available.

- [`scored_pool::Trait`](https://docs.rs/pallet-scored-pool/latest/pallet_scored_pool/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-scored-pool/latest/pallet_scored_pool/enum.Call.html)
- [`Module`](https://docs.rs/pallet-scored-pool/latest/pallet_scored_pool/struct.Module.html)

## Interface

### Public Functions

- `submit_candidacy` - Submit candidacy to become a member. Requires a deposit.
- `withdraw_candidacy` - Withdraw candidacy. Deposit is returned.
- `score` - Attribute a quantitative score to an entity.
- `kick` - Remove an entity from the pool and members. Deposit is returned.
- `change_member_count` - Changes the amount of candidates taken into `Members`.

## Usage

```rust
use frame_support::{decl_module, dispatch};
use frame_system::ensure_signed;
use pallet_scored_pool::{self as scored_pool};

pub trait Config: scored_pool::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		pub fn candidate(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let _ = <scored_pool::Module<T>>::submit_candidacy(
				T::Origin::from(Some(who.clone()).into())
			);
			Ok(())
		}
	}
}
```

## Dependencies

This module depends on the [System module](https://docs.rs/frame-system/latest/frame_system/).

License: Apache-2.0

# Session Module

The Session module allows validators to manage their session keys, provides a function for changing the session length, and handles session rotation.

- [`session::Trait`](https://docs.rs/pallet-session/latest/pallet_session/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-session/latest/pallet_session/enum.Call.html)
- [`Module`](https://docs.rs/pallet-session/latest/pallet_session/struct.Module.html)

## Overview

### Terminology

- **Session:** A session is a period of time that has a constant set of validators. Validators can only join or exit the validator set at a session change. It is measured in block numbers. The block where a session is ended is determined by the `ShouldEndSession` trait. When the session is ending, a new validator set can be chosen by `OnSessionEnding` implementations.
- **Session key:** A session key is actually several keys kept together that provide the various signing functions required by network authorities/validators in pursuit of their duties.
- **Validator ID:** Every account has an associated validator ID. For some simple staking systems, this may just be the same as the account ID. For staking systems using a stash/controller model, the validator ID would be the stash account ID of the controller.
- **Session key configuration process:** Session keys are set using `set_keys` for use not in the next session, but the session after next. They are stored in `NextKeys`, a mapping between the caller's `ValidatorId` and the session keys provided. `set_keys` allows users to set their session key prior to being selected as validator. It is a public call since it uses `ensure_signed`, which checks that the origin is a signed account. As such, the account ID of the origin stored in `NextKeys` may not necessarily be associated with a block author or a validator. The session keys of accounts are removed once their account balance is zero.
- **Session length:** This pallet does not assume anything about the length of each session. Rather, it relies on an implementation of `ShouldEndSession` to dictate a new session's start. This pallet provides the `PeriodicSessions` struct for simple periodic sessions.
- **Session rotation configuration:** Configure as either a 'normal' (rewardable session where rewards are applied) or 'exceptional' (slashable) session rotation.
- **Session rotation process:** At the beginning of each block, the `on_initialize` function queries the provided implementation of `ShouldEndSession`. If the session is to end the newly activated validator IDs and session keys are taken from storage and passed to the `SessionHandler`. The validator set supplied by `SessionManager::new_session` and the corresponding session keys, which may have been registered via `set_keys` during the previous session, are written to storage where they will wait one session before being passed to the `SessionHandler` themselves.

### Goals

The Session pallet is designed to make the following possible:

- Set session keys of the validator set for upcoming sessions.
- Control the length of sessions.
- Configure and switch between either normal or exceptional session rotations.

## Interface

### Dispatchable Functions

- `set_keys` - Set a validator's session keys for upcoming sessions.

### Public Functions

- `rotate_session` - Change to the next session. Register the new authority set. Queue changes for next session rotation.
- `disable_index` - Disable a validator by index.
- `disable` - Disable a validator by Validator ID

## Usage

### Example from the FRAME

The [Staking pallet](https://docs.rs/pallet-staking/latest/pallet_staking/) uses the Session pallet to get the validator set.

```rust
use pallet_session as session;

fn validators<T: pallet_session::Config>() -> Vec<<T as pallet_session::Config>::ValidatorId> {
	<pallet_session::Module<T>>::validators()
}
```

## Related Modules

- [Staking](https://docs.rs/pallet-staking/latest/pallet_staking/)

License: Apache-2.0

# Society Module

- [`society::Trait`](https://docs.rs/pallet-society/latest/pallet_society/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-society/latest/pallet_society/enum.Call.html)

## Overview

The Society module is an economic game which incentivizes users to participate and maintain a membership society.

### User Types

At any point, a user in the society can be one of a:

- Bidder - A user who has submitted intention of joining the society.
- Candidate - A user who will be voted on to join the society.
- Suspended Candidate - A user who failed to win a vote.
- Member - A user who is a member of the society.
- Suspended Member - A member of the society who has accumulated too many strikes or failed their membership challenge.

Of the non-suspended members, there is always a:

- Head - A member who is exempt from suspension.
- Defender - A member whose membership is under question and voted on again.

Of the non-suspended members of the society, a random set of them are chosen as "skeptics". The mechanics of skeptics is explained in the [member phase](https://docs.rs/pallet-society/latest/pallet_society/#member-phase) below.

### Mechanics

#### Rewards

Members are incentivized to participate in the society through rewards paid by the Society treasury. These payments have a maturity period that the user must wait before they are able to access the funds.

#### Punishments

Members can be punished by slashing the reward payouts that have not been collected. Additionally, members can accumulate "strikes", and when they reach a max strike limit, they become suspended.

#### Skeptics

During the voting period, a random set of members are selected as "skeptics". These skeptics are expected to vote on the current candidates. If they do not vote, their skeptic status is treated as a rejection vote, the member is deemed "lazy", and are given a strike per missing vote.

#### Membership Challenges

Every challenge rotation period, an existing member will be randomly selected to defend their membership into society. Then, other members can vote whether this defender should stay in society. A simple majority wins vote will determine the outcome of the user. Ties are treated as a failure of the challenge, but assuming no one else votes, the defender always get a free vote on their own challenge keeping them in the society. The Head member is exempt from the negative outcome of a membership challenge.

#### Society Treasury

The membership society is independently funded by a treasury managed by this module. Some subset of this treasury is placed in a Society Pot, which is used to determine the number of accepted bids.

#### Rate of Growth

The membership society can grow at a rate of 10 accepted candidates per rotation period up to the max membership threshold. Once this threshold is met, candidate selections are stalled until there is space for new members to join. This can be resolved by voting out existing members through the random challenges or by using governance to increase the maximum membership count.

### User Life Cycle

A user can go through the following phases:

```rust
          +------->  User  <----------+
          |           +               |
          |           |               |
+----------------------------------------------+
|         |           |               |        |
|         |           v               |        |
|         |        Bidder <-----------+        |
|         |           +               |        |
|         |           |               +        |
|         |           v            Suspended   |
|         |       Candidate +----> Candidate   |
|         |           +               +        |
|         |           |               |        |
|         +           |               |        |
|   Suspended +------>|               |        |
|      Member         |               |        |
|         ^           |               |        |
|         |           v               |        |
|         +-------+ Member <----------+        |
|                                              |
|                                              |
+------------------Society---------------------+
```

#### Initialization

The society is initialized with a single member who is automatically chosen as the Head.

#### Bid Phase

New users must have a bid to join the society.

A user can make a bid by reserving a deposit. Alternatively, an already existing member can create a bid on a user's behalf by "vouching" for them.

A bid includes reward information that the user would like to receive for joining the society. A vouching bid can additionally request some portion of that reward as a tip to the voucher for vouching for the prospective candidate.

Every rotation period, Bids are ordered by reward amount, and the module selects as many bids the Society Pot can support for that period.

These selected bids become candidates and move on to the Candidate phase. Bids that were not selected stay in the bidder pool until they are selected or a user chooses to "unbid".

#### Candidate Phase

Once a bidder becomes a candidate, members vote whether to approve or reject that candidate into society. This voting process also happens during a rotation period.

The approval and rejection criteria for candidates are not set on chain, and may change for different societies.

At the end of the rotation period, we collect the votes for a candidate and randomly select a vote as the final outcome.

```rust
 [ a-accept, r-reject, s-skeptic ]
+----------------------------------+
|                                  |
|  Member   |0|1|2|3|4|5|6|7|8|9|  |
|  -----------------------------   |
|  Vote     |a|a|a|r|s|r|a|a|s|a|  |
|  -----------------------------   |
|  Selected | | | |x| | | | | | |  |
|                                  |
+----------------------------------+

Result: Rejected
```

Each member that voted opposite to this randomly selected vote is punished by slashing their unclaimed payouts and increasing the number of strikes they have.

These slashed funds are given to a random user who voted the same as the selected vote as a reward for participating in the vote.

If the candidate wins the vote, they receive their bid reward as a future payout. If the bid was placed by a voucher, they will receive their portion of the reward, before the rest is paid to the winning candidate.

One winning candidate is selected as the Head of the members. This is randomly chosen, weighted by the number of approvals the winning candidates accumulated.

If the candidate loses the vote, they are suspended and it is up to the Suspension Judgement origin to determine if the candidate should go through the bidding process again, should be accepted into the membership society, or rejected and their deposit slashed.

#### Member Phase

Once a candidate becomes a member, their role is to participate in society.

Regular participation involves voting on candidates who want to join the membership society, and by voting in the right way, a member will accumulate future payouts. When a payout matures, members are able to claim those payouts.

Members can also vouch for users to join the society, and request a "tip" from the fees the new member would collect by joining the society. This vouching process is useful in situations where a user may not have enough balance to satisfy the bid deposit. A member can only vouch one user at a time.

During rotation periods, a random group of members are selected as "skeptics". These skeptics are expected to vote on the current candidates. If they do not vote, their skeptic status is treated as a rejection vote, the member is deemed "lazy", and are given a strike per missing vote.

There is a challenge period in parallel to the rotation period. During a challenge period, a random member is selected to defend their membership to the society. Other members make a traditional majority-wins vote to determine if the member should stay in the society. Ties are treated as a failure of the challenge.

If a member accumulates too many strikes or fails their membership challenge, they will become suspended. While a member is suspended, they are unable to claim matured payouts. It is up to the Suspension Judgement origin to determine if the member should re-enter society or be removed from society with all their future payouts slashed.

## Interface

### Dispatchable Functions

#### For General Users

- `bid` - A user can make a bid to join the membership society by reserving a deposit.
- `unbid` - A user can withdraw their bid for entry, the deposit is returned.

#### For Members

- `vouch` - A member can place a bid on behalf of a user to join the membership society.
- `unvouch` - A member can revoke their vouch for a user.
- `vote` - A member can vote to approve or reject a candidate's request to join the society.
- `defender_vote` - A member can vote to approve or reject a defender's continued membership to the society.
- `payout` - A member can claim their first matured payment.
- `unfound` - Allow the founder to unfound the society when they are the only member.

#### For Super Users

- `found` - The founder origin can initiate this society. Useful for bootstrapping the Society pallet on an already running chain.
- `judge_suspended_member` - The suspension judgement origin is able to make judgement on a suspended member.
- `judge_suspended_candidate` - The suspension judgement origin is able to make judgement on a suspended candidate.
- `set_max_membership` - The ROOT origin can update the maximum member count for the society. The max membership count must be greater than 1.

License: Apache-2.0

# Staking Module

The Staking module is used to manage funds at stake by network maintainers.

- [`staking::Trait`](https://docs.rs/pallet-staking/latest/pallet_staking/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html)
- [`Module`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Module.html)

## Overview

The Staking module is the means by which a set of network maintainers (known as *authorities* in some contexts and *validators* in others) are chosen based upon those who voluntarily place funds under deposit. Under deposit, those funds are rewarded under normal operation but are held at pain of *slash* (expropriation) should the staked maintainer be found not to be discharging its duties properly.

### Terminology

- Staking: The process of locking up funds for some time, placing them at risk of slashing (loss) in order to become a rewarded maintainer of the network.
- Validating: The process of running a node to actively maintain the network, either by producing blocks or guaranteeing finality of the chain.
- Nominating: The process of placing staked funds behind one or more validators in order to share in any reward, and punishment, they take.
- Stash account: The account holding an owner's funds used for staking.
- Controller account: The account that controls an owner's funds for staking.
- Era: A (whole) number of sessions, which is the period that the validator set (and each validator's active nominator set) is recalculated and where rewards are paid out.
- Slash: The punishment of a staker by reducing its funds.

### Goals

The staking system in Substrate NPoS is designed to make the following possible:

- Stake funds that are controlled by a cold wallet.
- Withdraw some, or deposit more, funds without interrupting the role of an entity.
- Switch between roles (nominator, validator, idle) with minimal overhead.

### Scenarios

#### Staking

Almost any interaction with the Staking module requires a process of ***bonding*** (also known as being a *staker*). To become *bonded*, a fund-holding account known as the *stash account*, which holds some or all of the funds that become frozen in place as part of the staking process, is paired with an active **controller** account, which issues instructions on how they shall be used.

An account pair can become bonded using the [`bond`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.bond) call.

Stash accounts can change their associated controller using the [`set_controller`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.set_controller) call.

There are three possible roles that any staked account pair can be in: `Validator`, `Nominator` and `Idle` (defined in [`StakerStatus`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.StakerStatus.html)). There are three corresponding instructions to change between roles, namely: [`validate`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.validate), [`nominate`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.nominate), and [`chill`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.chill).

#### Validating

A **validator** takes the role of either validating blocks or ensuring their finality, maintaining the veracity of the network. A validator should avoid both any sort of malicious misbehavior and going offline. Bonded accounts that state interest in being a validator do NOT get immediately chosen as a validator. Instead, they are declared as a *candidate* and they *might* get elected at the *next era* as a validator. The result of the election is determined by nominators and their votes.

An account can become a validator candidate via the [`validate`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.validate) call.

#### Nomination

A **nominator** does not take any *direct* role in maintaining the network, instead, it votes on a set of validators to be elected. Once interest in nomination is stated by an account, it takes effect at the next election round. The funds in the nominator's stash account indicate the *weight* of its vote. Both the rewards and any punishment that a validator earns are shared between the validator and its nominators. This rule incentivizes the nominators to NOT vote for the misbehaving/offline validators as much as possible, simply because the nominators will also lose funds if they vote poorly.

An account can become a nominator via the [`nominate`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.nominate) call.

#### Rewards and Slash

The **reward and slashing** procedure is the core of the Staking module, attempting to *embrace valid behavior* while *punishing any misbehavior or lack of availability*.

Rewards must be claimed for each era before it gets too old by `$HISTORY_DEPTH` using the `payout_stakers` call. Any account can call `payout_stakers`, which pays the reward to the validator as well as its nominators. Only the [`Config::MaxNominatorRewardedPerValidator`] biggest stakers can claim their reward. This is to limit the i/o cost to mutate storage for each nominator's account.

Slashing can occur at any point in time, once misbehavior is reported. Once slashing is determined, a value is deducted from the balance of the validator and all the nominators who voted for this validator (values are deducted from the *stash* account of the slashed entity).

Slashing logic is further described in the documentation of the `slashing` module.

Similar to slashing, rewards are also shared among a validator and its associated nominators. Yet, the reward funds are not always transferred to the stash account and can be configured. See [Reward Calculation](https://docs.rs/pallet-staking/latest/pallet_staking/#reward-calculation) for more details.

#### Chilling

Finally, any of the roles above can choose to step back temporarily and just chill for a while. This means that if they are a nominator, they will not be considered as voters anymore and if they are validators, they will no longer be a candidate for the next election.

An account can step back via the [`chill`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.chill) call.

### Session managing

The module implement the trait `SessionManager`. Which is the only API to query new validator set and allowing these validator set to be rewarded once their era is ended.

## Interface

### Dispatchable Functions

The dispatchable functions of the Staking module enable the steps needed for entities to accept and change their role, alongside some helper functions to get/set the metadata of the module.

### Public Functions

The Staking module contains many public storage items and (im)mutable functions.

## Usage

### Example: Rewarding a validator by id.

```rust
use frame_support::{decl_module, dispatch};
use frame_system::ensure_signed;
use pallet_staking::{self as staking};

pub trait Config: staking::Config {}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        /// Reward a validator.
        #[weight = 0]
        pub fn reward_myself(origin) -> dispatch::DispatchResult {
            let reported = ensure_signed(origin)?;
            <staking::Module<T>>::reward_by_ids(vec![(reported, 10)]);
            Ok(())
        }
    }
}
```

## Implementation Details

### Era payout

The era payout is computed using yearly inflation curve defined at [`T::RewardCurve`](https://docs.rs/pallet-staking/latest/pallet_staking/trait.Trait.html#associatedtype.RewardCurve) as such:

```nocompile
staker_payout = yearly_inflation(npos_token_staked / total_tokens) * total_tokens / era_per_year
```

This payout is used to reward stakers as defined in next section

```nocompile
remaining_payout = max_yearly_inflation * total_tokens / era_per_year - staker_payout
```

The remaining reward is send to the configurable end-point [`T::RewardRemainder`](https://docs.rs/pallet-staking/latest/pallet_staking/trait.Trait.html#associatedtype.RewardRemainder).

### Reward Calculation

Validators and nominators are rewarded at the end of each era. The total reward of an era is calculated using the era duration and the staking rate (the total amount of tokens staked by nominators and validators, divided by the total token supply). It aims to incentivize toward a defined staking rate. The full specification can be found [here](https://research.web3.foundation/en/latest/polkadot/economics/1-token-economics.html#inflation-model).

Total reward is split among validators and their nominators depending on the number of points they received during the era. Points are added to a validator using [`reward_by_ids`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.reward_by_ids) or [`reward_by_indices`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.reward_by_indices).

[`Module`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Module.html) implements [`pallet_authorship::EventHandler`](https://docs.rs/pallet-authorship/latest/pallet_authorship/trait.EventHandler.html) to add reward points to block producer and block producer of referenced uncles.

The validator and its nominator split their reward as following:

The validator can declare an amount, named [`commission`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.ValidatorPrefs.html#structfield.commission), that does not get shared with the nominators at each reward payout through its [`ValidatorPrefs`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.ValidatorPrefs.html). This value gets deducted from the total reward that is paid to the validator and its nominators. The remaining portion is split among the validator and all of the nominators that nominated the validator, proportional to the value staked behind this validator (*i.e.* dividing the [`own`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Exposure.html#structfield.own) or [`others`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Exposure.html#structfield.others) by [`total`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Exposure.html#structfield.total) in [`Exposure`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Exposure.html)).

All entities who receive a reward have the option to choose their reward destination through the [`Payee`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.Payee.html) storage item (see [`set_payee`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.set_payee)), to be one of the following:

- Controller account, (obviously) not increasing the staked value.
- Stash account, not increasing the staked value.
- Stash account, also increasing the staked value.

### Additional Fund Management Operations

Any funds already placed into stash can be the target of the following operations:

The controller account can free a portion (or all) of the funds using the [`unbond`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.unbond) call. Note that the funds are not immediately accessible. Instead, a duration denoted by [`BondingDuration`](https://docs.rs/pallet-staking/latest/pallet_staking/trait.Trait.html#associatedtype.BondingDuration) (in number of eras) must pass until the funds can actually be removed. Once the `BondingDuration` is over, the [`withdraw_unbonded`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.withdraw_unbonded) call can be used to actually withdraw the funds.

Note that there is a limitation to the number of fund-chunks that can be scheduled to be unlocked in the future via [`unbond`](https://docs.rs/pallet-staking/latest/pallet_staking/enum.Call.html#variant.unbond). In case this maximum (`MAX_UNLOCKING_CHUNKS`) is reached, the bonded account *must* first wait until a successful call to `withdraw_unbonded` to remove some of the chunks.

### Election Algorithm

The current election algorithm is implemented based on Phragmén. The reference implementation can be found [here](https://github.com/w3f/consensus/tree/master/NPoS).

The election algorithm, aside from electing the validators with the most stake value and votes, tries to divide the nominator votes among candidates in an equal manner. To further assure this, an optional post-processing can be applied that iteratively normalizes the nominator staked values until the total difference among votes of a particular nominator are less than a threshold.

## GenesisConfig

The Staking module depends on the [`GenesisConfig`](https://docs.rs/pallet-staking/latest/pallet_staking/struct.GenesisConfig.html). The `GenesisConfig` is optional and allow to set some initial stakers.

## Related Modules

- [Balances](https://docs.rs/pallet-balances/latest/pallet_balances/): Used to manage values at stake.
- [Session](https://docs.rs/pallet-session/latest/pallet_session/): Used to manage sessions. Also, a list of new validators is stored in the Session module's `Validators` at the end of each era.

License: Apache-2.0

# Sudo Module

- [`sudo::Trait`](https://docs.rs/pallet-sudo/latest/pallet_sudo/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-sudo/latest/pallet_sudo/enum.Call.html)

## Overview

The Sudo module allows for a single account (called the "sudo key") to execute dispatchable functions that require a `Root` call or designate a new account to replace them as the sudo key. Only one account can be the sudo key at a time.

## Interface

### Dispatchable Functions

Only the sudo key can call the dispatchable functions from the Sudo module.

- `sudo` - Make a `Root` call to a dispatchable function.
- `set_key` - Assign a new account to be the sudo key.

## Usage

### Executing Privileged Functions

The Sudo module itself is not intended to be used within other modules. Instead, you can build "privileged functions" (i.e. functions that require `Root` origin) in other modules. You can execute these privileged functions by calling `sudo` with the sudo key account. Privileged functions cannot be directly executed via an extrinsic.

Learn more about privileged functions and `Root` origin in the [`Origin`](https://docs.substrate.dev/docs/substrate-types) type documentation.

### Simple Code Snippet

This is an example of a module that exposes a privileged function:

```rust
use frame_support::{decl_module, dispatch};
use frame_system::ensure_root;

pub trait Config: frame_system::Config {}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
        pub fn privileged_function(origin) -> dispatch::DispatchResult {
            ensure_root(origin)?;

            // do something...

            Ok(())
        }
    }
}
```

## Genesis Config

The Sudo module depends on the [`GenesisConfig`](https://docs.rs/pallet-sudo/latest/pallet_sudo/struct.GenesisConfig.html). You need to set an initial superuser account as the sudo `key`.

## Related Modules

- [Democracy](https://docs.rs/pallet-democracy/latest/pallet_democracy/)

License: Apache-2.0

# Support 

Support code for the runtime.

License: Apache-2.0

# System Module

The System module provides low-level access to core types and cross-cutting utilities. It acts as the base layer for other pallets to interact with the Substrate framework components.

- [`system::Trait`](https://docs.rs/frame-system/latest/frame_system/trait.Trait.html)

## Overview

The System module defines the core data types used in a Substrate runtime. It also provides several utility functions (see [`Module`](https://docs.rs/frame-system/latest/frame_system/struct.Module.html)) for other FRAME pallets.

In addition, it manages the storage items for extrinsics data, indexes, event records, and digest items, among other things that support the execution of the current block.

It also handles low-level tasks like depositing logs, basic set up and take down of temporary storage entries, and access to previous block hashes.

## Interface

### Dispatchable Functions

The System module does not implement any dispatchable functions.

### Public Functions

See the [`Module`](https://docs.rs/frame-system/latest/frame_system/struct.Module.html) struct for details of publicly available functions.

### Signed Extensions

The System module defines the following extensions:

- [`CheckWeight`]: Checks the weight and length of the block and ensure that it does not exceed the limits.
- [`CheckNonce`]: Checks the nonce of the transaction. Contains a single payload of type `T::Index`.
- [`CheckEra`]: Checks the era of the transaction. Contains a single payload of type `Era`.
- [`CheckGenesis`]: Checks the provided genesis hash of the transaction. Must be a part of the signed payload of the transaction.
- [`CheckSpecVersion`]: Checks that the runtime version is the same as the one used to sign the transaction.
- [`CheckTxVersion`]: Checks that the transaction version is the same as the one used to sign the transaction.

Lookup the runtime aggregator file (e.g. `node/runtime`) to see the full list of signed extensions included in a chain.

## Usage

### Prerequisites

Import the System module and derive your module's configuration trait from the system trait.

### Example - Get extrinsic count and parent hash for the current block

```rust
use frame_support::{decl_module, dispatch};
use frame_system::{self as system, ensure_signed};

pub trait Config: system::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		pub fn system_module_example(origin) -> dispatch::DispatchResult {
			let _sender = ensure_signed(origin)?;
			let _extrinsic_count = <system::Module<T>>::extrinsic_count();
			let _parent_hash = <system::Module<T>>::parent_hash();
			Ok(())
		}
	}
}
```

License: Apache-2.0

# Timestamp Module

The Timestamp module provides functionality to get and set the on-chain time.

- [`timestamp::Trait`](https://docs.rs/pallet-timestamp/latest/pallet_timestamp/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-timestamp/latest/pallet_timestamp/enum.Call.html)
- [`Module`](https://docs.rs/pallet-timestamp/latest/pallet_timestamp/struct.Module.html)

## Overview

The Timestamp module allows the validators to set and validate a timestamp with each block.

It uses inherents for timestamp data, which is provided by the block author and validated/verified by other validators. The timestamp can be set only once per block and must be set each block. There could be a constraint on how much time must pass before setting the new timestamp.

**NOTE:** The Timestamp module is the recommended way to query the on-chain time instead of using an approach based on block numbers. The block number based time measurement can cause issues because of cumulative calculation errors and hence should be avoided.

## Interface

### Dispatchable Functions

- `set` - Sets the current time.

### Public functions

- `get` - Gets the current time for the current block. If this function is called prior to setting the timestamp, it will return the timestamp of the previous block.

### Config Getters

- `MinimumPeriod` - Gets the minimum (and advised) period between blocks for the chain.

## Usage

The following example shows how to use the Timestamp module in your custom module to query the current timestamp.

### Prerequisites

Import the Timestamp module into your custom module and derive the module configuration trait from the timestamp trait.

### Get current timestamp

```rust
use frame_support::{decl_module, dispatch};
use frame_system::ensure_signed;

pub trait Config: timestamp::Config {}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		#[weight = 0]
		pub fn get_time(origin) -> dispatch::DispatchResult {
			let _sender = ensure_signed(origin)?;
			let _now = <timestamp::Module<T>>::get();
			Ok(())
		}
	}
}
```

### Example from the FRAME

The [Session module](https://github.com/paritytech/substrate/blob/master/frame/session/src/lib.rs) uses the Timestamp module for session management.

## Related Modules

- [Session](https://docs.rs/pallet-session/latest/pallet_session/)

License: Apache-2.0

# Tipping Module ( pallet-tips )

**Note :: This pallet is tightly coupled to pallet-treasury**

A subsystem to allow for an agile "tipping" process, whereby a reward may be given without first having a pre-determined stakeholder group come to consensus on how much should be paid.

A group of `Tippers` is determined through the config `Trait`. After half of these have declared some amount that they believe a particular reported reason deserves, then a countdown period is entered where any remaining members can declare their tip amounts also. After the close of the countdown period, the median of all declared tips is paid to the reported beneficiary, along with any finders fee, in case of a public (and bonded) original report.

### Terminology

- **Tipping:** The process of gathering declarations of amounts to tip and taking the median amount to be transferred from the treasury to a beneficiary account.
- **Tip Reason:** The reason for a tip; generally a URL which embodies or explains why a particular individual (identified by an account ID) is worthy of a recognition by the treasury.
- **Finder:** The original public reporter of some reason for tipping.
- **Finders Fee:** Some proportion of the tip amount that is paid to the reporter of the tip, rather than the main beneficiary.

## Interface

### Dispatchable Functions

- `report_awesome` - Report something worthy of a tip and register for a finders fee.
- `retract_tip` - Retract a previous (finders fee registered) report.
- `tip_new` - Report an item worthy of a tip and declare a specific amount to tip.
- `tip` - Declare or redeclare an amount to tip for a particular reason.
- `close_tip` - Close and pay out a tip.



# Transaction Payment Module

This module provides the basic logic needed to pay the absolute minimum amount needed for a transaction to be included. This includes:

- *weight fee*: A fee proportional to amount of weight a transaction consumes.
- *length fee*: A fee proportional to the encoded length of the transaction.
- *tip*: An optional tip. Tip increases the priority of the transaction, giving it a higher chance to be included by the transaction queue.

Additionally, this module allows one to configure:

- The mapping between one unit of weight to one unit of fee via [`Config::WeightToFee`].
- A means of updating the fee for the next block, via defining a multiplier, based on the final state of the chain at the end of the previous block. This can be configured via [`Config::FeeMultiplierUpdate`]

License: Apache-2.0





# Treasury Module

The Treasury module provides a "pot" of funds that can be managed by stakeholders in the system and a structure for making spending proposals from this pot.

## Overview

The Treasury Module itself provides the pot to store funds, and a means for stakeholders to propose, approve, and deny expenditures. The chain will need to provide a method (e.g.inflation, fees) for collecting funds.

By way of example, the Council could vote to fund the Treasury with a portion of the block reward and use the funds to pay developers.

### Terminology

- **Proposal:** A suggestion to allocate funds from the pot to a beneficiary.
- **Beneficiary:** An account who will receive the funds from a proposal if the proposal is approved.
- **Deposit:** Funds that a proposer must lock when making a proposal. The deposit will be returned or slashed if the proposal is approved or rejected respectively.
- **Pot:** Unspent funds accumulated by the treasury module.

## Interface

### Dispatchable Functions

General spending/proposal protocol:

- `propose_spend` - Make a spending proposal and stake the required deposit.
- `reject_proposal` - Reject a proposal, slashing the deposit.
- `approve_proposal` - Accept the proposal, returning the deposit.



# Utility Module

A stateless module with helpers for dispatch management which does no re-authentication.

- [`utility::Trait`](https://docs.rs/pallet-utility/latest/pallet_utility/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-utility/latest/pallet_utility/enum.Call.html)

## Overview

This module contains two basic pieces of functionality:

- Batch dispatch: A stateless operation, allowing any origin to execute multiple calls in a single dispatch. This can be useful to amalgamate proposals, combining `set_code` with corresponding `set_storage`s, for efficient multiple payouts with just a single signature verify, or in combination with one of the other two dispatch functionality.
- Pseudonymal dispatch: A stateless operation, allowing a signed origin to execute a call from an alternative signed origin. Each account has 2 * 2**16 possible "pseudonyms" (alternative account IDs) and these can be stacked. This can be useful as a key management tool, where you need multiple distinct accounts (e.g. as controllers for many staking accounts), but where it's perfectly fine to have each of them controlled by the same underlying keypair. Derivative accounts are, for the purposes of proxy filtering considered exactly the same as the oigin and are thus hampered with the origin's filters.

Since proxy filters are respected in all dispatches of this module, it should never need to be filtered by any proxy.

## Interface

### Dispatchable Functions

#### For batch dispatch

- `batch` - Dispatch multiple calls from the sender's origin.

#### For pseudonymal dispatch

- `as_derivative` - Dispatch a call from a derivative signed origin.

License: Apache-2.0





# Vesting Module

- [`vesting::Trait`](https://docs.rs/pallet-vesting/latest/pallet_vesting/trait.Trait.html)
- [`Call`](https://docs.rs/pallet-vesting/latest/pallet_vesting/enum.Call.html)

## Overview

A simple module providing a means of placing a linear curve on an account's locked balance. This module ensures that there is a lock in place preventing the balance to drop below the *unvested* amount for any reason other than transaction fee payment.

As the amount vested increases over time, the amount unvested reduces. However, locks remain in place and explicit action is needed on behalf of the user to ensure that the amount locked is equivalent to the amount remaining to be vested. This is done through a dispatchable function, either `vest` (in typical case where the sender is calling on their own behalf) or `vest_other` in case the sender is calling on another account's behalf.

## Interface

This module implements the `VestingSchedule` trait.

### Dispatchable Functions

- `vest` - Update the lock, reducing it in line with the amount "vested" so far.
- `vest_other` - Update the lock of another account, reducing it in line with the amount "vested" so far.

License: Apache-2.0



