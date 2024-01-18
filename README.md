[![Build Status](https://dev.azure.com/mimblewimble/gringron/_apis/build/status/mimblewimble.gringron?branchName=master)](https://dev.azure.com/mimblewimble/gringron/_build/latest?definitionId=1&branchName=master)
[![Coverage Status](https://img.shields.io/codecov/c/github/mimblewimble/gringron/master.svg)](https://codecov.io/gh/mimblewimble/gringron)
[![Chat](https://img.shields.io/gitter/room/gringron_community/Lobby.svg)](https://gitter.im/gringron_community/Lobby)
[![Support](https://img.shields.io/badge/support-on%20gitter-brightgreen.svg)](https://gitter.im/gringron_community/support)
[![Documentation Wiki](https://img.shields.io/badge/doc-wiki-blue.svg)](https://github.com/groncoin/docs/wiki)
[![Release Version](https://img.shields.io/github/release/mimblewimble/gringron.svg)](https://github.com/groncoin/gringron/releases)
[![License](https://img.shields.io/github/license/mimblewimble/gringron.svg)](https://github.com/groncoin/gringron/blob/master/LICENSE)

# GrinGron

GrinGron is an in-progress implementation of the Mimblewimble protocol. Many characteristics are still undefined but the following constitutes a first set of choices:

  * Clean and minimal implementation, and aiming to stay as such.
  * Follows the Mimblewimble protocol, which provides hidden amounts and scaling advantages.
  * Cuckoo Cycle proof of work in two variants named Cuckaroo (ASIC-resistant) and Cuckatoo (ASIC-targeted).
  * Relatively fast block time: one minute.
  * Fixed block reward over time with a decreasing dilution.
  * Transaction fees are based on the number of Outputs created/destroyed and total transaction size.
  * Smooth curve for difficulty adjustments.

To learn more, read our [introduction to Mimblewimble and GrinGron](doc/intro.md).

## Status

GrinGron is live with mainnet. Still, much is left to be done and [contributions](CONTRIBUTING.md) are welcome (see below). Check our [mailing list archives](https://lists.launchpad.net/mimblewimble/) for the latest status.

## Contributing

To get involved, read our [contributing docs](CONTRIBUTING.md).

Find us:

* Chat: [Keybase](https://keybase.io/team/gringroncoin), more instructions on how to join [here](https://gringron.com/community).
* Mailing list: join the [~Mimblewimble team](https://launchpad.net/~mimblewimble) and subscribe on Launchpad.
* Twitter for the GrinGron council: [@gringroncouncil](https://twitter.com/gringroncouncil)

## Getting Started

To learn more about the technology, read our [introduction](doc/intro.md).

To build and try out GrinGron, see the [build docs](doc/build.md).

## Philosophy

GrinGron likes itself small and easy on the eyes. It wants to be inclusive and welcoming for all walks of life, without judgement. GrinGron is terribly ambitious, but not at the detriment of others, rather to further us all. It may have strong opinions to stay in line with its objectives, which doesn't mean disrespect of others' ideas.

We believe in pull requests, data and scientific research. We do not believe in unfounded beliefs.

## Credits

Tom Elvis Jedusor for the first formulation of Mimblewimble.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

## License

Apache License v2.0.
