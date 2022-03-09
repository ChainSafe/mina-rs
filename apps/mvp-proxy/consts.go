package main

import (
	"fmt"

	"github.com/libp2p/go-libp2p"
	"golang.org/x/crypto/blake2b"

	libp2pmplex "github.com/libp2p/go-libp2p-mplex"
)

const (
	BUFFER_SIZE = 1024 * 1024
	// mainnet
	CHAIN_ID = "5f704cc0c82e0ed70e873f0893d7e06f148524e3f0bdae2afb02e7819a0c24d1"

	// mainnet seeds: https://storage.googleapis.com/mina-seed-lists/mainnet_seeds.txt
	// devnet seeds:  https://storage.googleapis.com/seed-lists/devnet_seeds.txt
	// MINA_NODE_ADDRESS = "/ip4/127.0.0.1/tcp/8302/p2p/12D3KooWKK3RpV1MWAZk3FJ5xqbVPL2BMDdUEGSfwfQoUprBNZCv"
	// MINA_NODE_ADDRESS = "/ip4/95.217.106.189/tcp/8302/p2p/12D3KooWSxxCtzRLfUzoxgRYW9fTKWPUujdvStuwCPSPUN3629mb"
	// MINA_NODE_ADDRESS = "/dns4/seed-1.mainnet.o1test.net/tcp/10000/p2p/12D3KooWCa1d7G3SkRxy846qTvdAFX69NnoYZ32orWVLqJcDVGHW"
)

var (
	SEEDS = []string{
		"/dns4/seed-1.mainnet.o1test.net/tcp/10000/p2p/12D3KooWCa1d7G3SkRxy846qTvdAFX69NnoYZ32orWVLqJcDVGHW",
		"/dns4/seed-2.mainnet.o1test.net/tcp/10001/p2p/12D3KooWK4NfthViCTyLgVQa1WvqDC1NccVxGruCXCZUt3GqvFvn",
		"/dns4/seed-3.mainnet.o1test.net/tcp/10002/p2p/12D3KooWNofeYVAJXA3WGg2qCDhs3GEe71kTmKpFQXRbZmCz1Vr7",
		"/dns4/mina-seed.bitcat365.com/tcp/10001/p2p/12D3KooWQzozNTDKL7MqUh6Nh11GMA4pQhRCAsNTRWxCAzAi4VbE",
		"/dns4/mina-seed-1.zkvalidator.com/tcp/8302/p2p/12D3KooWSR7LMBSfEk3LQUudmsX27yuRHe9NUxwLumurGF5P1MNS",
		"/dns4/mina-1.figment.io/tcp/8302/p2p/12D3KooWSkfwArLtqGMht1a9w3z3QiiqA2E6seBRAk378rvanGRZ",
		"/dns4/mina-seed.staker.space/tcp/8302/p2p/12D3KooWCE97fGwuDCicVNK3ZWF8fVzfNezp3uGjmSc8VrRFem6a",
		"/dns4/mina-seed.genesislab.net/tcp/8302/p2p/12D3KooWRcHiFQsbYgjPSxtMg4Y9ifrvmCFtJQ8Qztqd3z4L9buU",
		"/dns4/mina-seed.hashquark.io/tcp/8302/p2p/12D3KooWRqdbJszoX6AB2E47KR45Kex1RptichA2MDkNSCqX5eb4",
		"/ip4/95.217.106.189/tcp/8302/p2p/12D3KooWSxxCtzRLfUzoxgRYW9fTKWPUujdvStuwCPSPUN3629mb",
		"/dns4/mina.cloud.p2pvalidator.org/tcp/8302/p2p/12D3KooW9qa8CcihmpPbKjN1e8da1RsBS67bExgpVDD9sCjzbHfh",
		"/dns4/mina-seed.dsrvlabs.net/tcp/8302/p2p/12D3KooWFTrtiuscobTsJwvShNzBWH56Jt6hWoZTtYqFFyQWFA7c",
		"/dns4/mina-seed-1.nodeasy.com/tcp/10001/p2p/12D3KooWRMXtoYktAqkNFd9LkT1XpAJWryqje88owWf9v9SpaayN",
		"/dns4/earth.mina.kelepool.pro/tcp/8302/p2p/12D3KooWSBRhKVd9r1JXkRTD4qc9SkNd9ACrRCeW9e6GcDakqHjh",
		"/dns4/seed.minaprotocol.fish/tcp/8302/p2p/12D3KooWQHTEXCbS1xxEMFHdALBTA1uLbFPr3okXvUos57d5seHW",
		"/ip4/159.89.96.164/tcp/8302/p2p/12D3KooWCCZkMjQxsBsSLPmAvFC9RGLz4XjKtdvpKmBdr5zpYz6x",
		"/dns4/seed.mina-staked.cloud/tcp/8302/p2p/12D3KooWNbeghjwB9MKgVniTv4pqtCbtHxjWpidiaoRiMhow3Mr1",
		"/ip4/47.242.110.4/tcp/8302/p2p/12D3KooWKsgKQRNsptXF7MDJ37FzJ9sz6uACuzow6zTJkyog3bWq",
		"/dns4/seed.minaexplorer.com/tcp/8302/p2p/12D3KooWR7coZtrMHvsgsfiWq2GESYypac3i29LFGp6EpbtjxBiJ",
		"/ip4/135.181.63.89/tcp/8302/p2p/12D3KooWNtvMGAvzrDEPBAHhiB7YoSWjPgmLcMmEeLCeoomWT8bT",
		"/ip4/135.181.132.212/tcp/8302/p2p/12D3KooWDGUWGrbHWrzft9iASSeytvbU9QmzxMavxAqqDfz8e7xE",
		"/dns4/seed.piconbello.com/tcp/10001/p2p/12D3KooWRFac2AztcTeen2DYNwnTrmVBvwNDsRiFpDVdTkwdFAHP",
		"/dns4/mina-seed.w3m.one/tcp/10001/p2p/12D3KooWFVvahnR3ofaSNX5XZaUQJ1zbrySjNCJP8K1vjBhHWURB",
	}
)

var (
	muxer = libp2p.Muxer("/coda/mplex/1.0.0", libp2pmplex.DefaultTransport)
	// https://github.com/MinaProtocol/mina/issues/9043
	// rendezvousString = fmt.Sprintf("/coda/0.0.1/%s", "971ab7c2a9370f90a3e4f5d1e711d6428e4d7ff210cbb7b5563d70843ce95274")
	rendezvousString     = fmt.Sprintf("/coda/0.0.1/%s", CHAIN_ID)
	mdnsRendezvousString = "_coda-discovery._udp.local"
	pnetKey              = blake2b.Sum256([]byte(rendezvousString))
	userAgent            = libp2p.UserAgent("github.com/codaprotocol/coda/tree/master/src/app/libp2p_helper")
	privateNetwork       = libp2p.PrivateNetwork(pnetKey[:])
)
