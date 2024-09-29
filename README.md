<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About
WeaveVM Archiver is an ETL archive pipeline for EVM networks. It's the simplest way to interface with WeaveVM's permanent data feature without smart contract redeployments.

## About SEI Network
The [SEI V2 Mainnet](https://blog.sei.io/sei-v2-the-first-parallelized-evm/) is a decentralized network built on top of a hyper optimized execution layer, making it the first the first parallelized EVM Blockchain .

## sei-wvm-archiver node configuration

- node endpoint: https://sei.wvm.network
- wvm-archiver node version: [v0.2.4](https://github.com/weaveVM/wvm-archiver/releases/tag/v0.2.4)

### Node configuration

```json
{
    "name": "SEI V2 Mainnet",
    "network_chain_id": 1329,
    "wvm_chain_id": 9496,
    "network_rpc": "https://evm-rpc.sei-apis.com",
    "wvm_rpc": "https://testnet-rpc.wvm.dev",
    "block_time": 0.395,
    "start_block": 105246036,
    "archiver_address": "0x2D76d7B140d078C575eAAD109168c606FE9d506C",
    "backfill_address": "0x55dA54ee977FBe734d5250F0558bc4B2FBe36b2a",
    "archive_pool_address": "0x0000000000000000000000000000000000000000"
}
```

# Generic documentation content from [wvm-archiver](https://github.com/weaveVM/wvm-archiver) repo

### WeaveVM Archiver Usage

WeaveVM Archiver is the ideal choice if you want to:

- Interface with WeaveVM's permanent data settlement and high-throughput DA
- Maintain your current data settlement or DA architecture
- Have an interface with WeaveVM without rollup smart contract redeployments
- Avoid codebase refactoring

## Build & Run

```bash
git clone https://github.com/weaveVM/wvm-archiver.git

cd wvm-archiver

cargo shuttle run
```

### Prerequisites & Dependencies

While a WeaveVM Archiver node can run without web2 component dependencies, this node implementation uses [planetscale](https://planetscale.com) for cloud indexing (indexing target network block ID to WVM archive TXID) and [shuttle.rs](https://shuttle.rs) for backend hosting. Check [.env.example](./env.example) to set up your environment variables.

```js
archiver_pk="" // WeaveVM archiver PK
backfill_pk="" // WeaveVM backfill PK
backfill_start_block="0" // it defaults to 0 (genesis), but it's dynamic, so you can specify from which block number you want to start backfilling
network="./networks/your_network.json"

DATABASE_HOST="" // planetscale
DATABASE_USERNAME="" // planetscale
DATABASE_PASSWORD="" // planetscale
```

### Add Your Network

To start archiving your network block data on WeaveVM:

1. Add your network config file to the [networks](./networks/) directory.
2. Name your config file using snake_case syntax (e.g., `your_network_name.json`).
3. Modify properties that don't have a `wvm_` prefix in the config JSON file. Check [_template.json](./networks/_template.json) guide
4. Fund your `archiver_address` & `backfill_address` with a sufficient amount of tWVM (1 MB costs ~ 5 cents). Check out [WVM Faucet](https://wvm.dev/faucet) to claim $tWVM. Make sure that the two addresses are distinct.
5. Choose a unique `archive_pool_address` that's different from your `archiver_address` & `backfill_address`
6. set `start_block` value to the most recent network's blockheight. That will facilitate the archiver to start in sync with live blockheight while, in parallel, reindexing from genesis using the `backfill_address`. 
7. Set up your PlanetScale DB according to `db_schema.sql`.

#### Parallel Threads of Archiving

As mentioned previously, `archiver_address` is responsible for archiving blocks starting from the `start_block` specified in your `network.json` config file, while also keeping up with the network’s current blockheight (live sync). Meanwhile, `backfill_address` handles archiving blocks from `backfill_start_block` up to `start_block`.

```txt
backfill thread: backfill_start_block -> start_block
live sync thread: start_block -> network's live blockheight
```

### RPC Proxy and Caching

You can use [eRPC](https://github.com/erpc/erpc) to cache, load-balance and failover between as many RPC endpoints and use eRPC's proxy URL in each network's config for WeaveVM. This will increase performance and resiliency and reduce RPC usage cost while fetching network's block data via WeaveVM.

```bash
# modify erpc.yaml
cp erpc.yaml.dist erpc.yaml
code erpc.yaml

# run docker-compose
docker-compose up -d
```

Finally, you can set eRPC's proxy URL in each relative network config.

```optimism.json
{
    "name": "Optimism",
    "network_chain_id": 10,
    "network_rpc": "http://erpc:4000/main/evm/10",
    ...
}
```

## How it works

The WeaveVM Archiver node operates as follows:

1. It starts downloading the target EVM network block data from the RPC you provide in the network config file.
2. The node begins pulling blocks from the `start_block` defined in the network's config file.
3. The block data is then serialized in [borsh](https://borsh.io) format and compressed using Brotli.
4. The serialized-compressed data is pushed to WeaveVM as calldata transaction from the `archiver_address` & `backfill_address` to the `archive_pool_address`.
5. Simultaneously, the resulting TXID from pushing data to WeaveVM and the archived EVM block ID are indexed in the cloud for faster data retrieval.

## Server Methods

As mentioned, PlanetScale is used for cloud indexing, which allows a WeaveVM Archiver node to expose its WeaveVM data as a RESTful API.

#### Node instance endpoint: https://metis.wvm.network

### WeaveVM Archiver node instance info

```bash
curl -X GET https://the_network.wvm.network/info
```
**returns:**

```rs
pub struct InfoServerResponse {
    first_archived_block: Option<u64>,
    last_archived_block: Option<u64>,
    livesync_start_block: u64,
    total_archived_blocks: u64,
    blocks_behind_live_blockheight: u64,
    archiver_balance: U256,
    archiver_address: String,
    backfill_address: String,
    backfill_balance: U256,
    network_name: String,
    network_chain_id: u32,
    network_rpc: String,
}
```

### Retrieve the WVM archive TXID for a given EVM block ID

```bash
curl -X GET https://the_network.wvm.network/block/$BLOCK_ID
```

### Decode the WVM archived block data for a given EVM block ID (return original block data in JSON format)

```bash
curl -X GET https://the_network.wvm.network/block/raw/$BLOCK_ID
```

## License
This project is licensed under the [BSL 1.1 License](./LICENSE)