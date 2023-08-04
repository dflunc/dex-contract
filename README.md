# Classic DFSwap

This project is a fork of [Terraswap](https://github.com/terraswap/classic-terraswap), with the only modification being the fee distribution logic. It allocates one-third of the 0.3% fee to the LP stakers at [DFLunc project](https://www.dflunc.app/stakeLP), while keeping the rest of the business logic consistent with Terraswap.

## Contracts

| Name                                               | Description                                  |
| -------------------------------------------------- | -------------------------------------------- |
| [`dfluncswap_factory`](contracts/dfluncswap_factory) |                                              |
| [`dfluncswap_pair`](contracts/dfluncswap_pair)       |                                              |
| [`dfluncswap_router`](contracts/dfluncswap_router)   |                                              |
| [`dfluncswap_token`](contracts/dfluncswap_token)     | CW20 (ERC20 equivalent) token implementation |

* dfluncswap_factory

   Classic: `terra1xk0p3jpxmw70dt0xntyf0h73ks8ap5uravp4pruy0jherjpnnneq7z2xnx`
