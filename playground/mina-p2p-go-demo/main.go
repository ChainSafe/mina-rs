package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p-core/peer"
	"github.com/libp2p/go-libp2p-core/peerstore"
	"golang.org/x/crypto/blake2b"

	// relayv1 "github.com/libp2p/go-libp2p/p2p/protocol/circuitv1/relay"
	// relayv2 "github.com/libp2p/go-libp2p/p2p/protocol/circuitv2/relay"

	libp2pmplex "github.com/libp2p/go-libp2p-mplex"
	mplex "github.com/libp2p/go-mplex"
	ma "github.com/multiformats/go-multiaddr"
)

func main() {
	run()
}

var (
	muxer = libp2p.Muxer("/coda/mplex/1.0.0", libp2pmplex.DefaultTransport)
)

func run() {
	mplex.MaxMessageSize = 1 << 30
	// https://github.com/MinaProtocol/mina/issues/9043
	// rendezvousString := fmt.Sprintf("/coda/0.0.1/%s", "971ab7c2a9370f90a3e4f5d1e711d6428e4d7ff210cbb7b5563d70843ce95274")
	rendezvousString := fmt.Sprintf("/coda/0.0.1/%s", "5f704cc0c82e0ed70e873f0893d7e06f148524e3f0bdae2afb02e7819a0c24d1")
	pnetKey := blake2b.Sum256([]byte(rendezvousString))

	h1, err := libp2p.New(
		context.Background(),
		libp2p.EnableRelay(),
		muxer,
		libp2p.UserAgent("github.com/codaprotocol/coda/tree/master/src/app/libp2p_helper"),
		libp2p.PrivateNetwork(pnetKey[:]),
	)
	if err != nil {
		log.Printf("Failed to create h1: %v", err)
		return
	}
	log.Printf("h1: %s %s\n", h1.Addrs(), h1.ID())
	{
		// addrStr := "/ip4/127.0.0.1/tcp/8302/p2p/12D3KooWPsVYHpYQxYkfbDCJcHqxzpQfAeGGLF5nk7ESrBLScq5S"

		// mainnet: https://storage.googleapis.com/mina-seed-lists/mainnet_seeds.txt
		// devnet:  https://storage.googleapis.com/seed-lists/devnet_seeds.txt
		// addrStr := "/ip4/47.242.110.4/tcp/8302/p2p/12D3KooWFVn7SZzTsS8btkLMWUE14PqfimZw23pbntTacaMk4Rc2"
		addrStr := "/ip4/95.217.106.189/tcp/8302/p2p/12D3KooWSxxCtzRLfUzoxgRYW9fTKWPUujdvStuwCPSPUN3629mb"
		mtaddr, _ := ma.NewMultiaddr(addrStr)
		log.Printf("Connecting to mina: %s", mtaddr)
		info, err := peer.AddrInfoFromP2pAddr(mtaddr)
		if err != nil {
			log.Println(err)
			return
		}
		h1.Peerstore().AddAddrs(info.ID, info.Addrs, peerstore.ConnectedAddrTTL)
		s, err := h1.NewStream(context.Background(), info.ID, "/mina/node-status")
		if err != nil {
			log.Println("huh, this should have worked: ", err)
			return
		}
		time.Sleep(time.Second * 1)
		// 1 megabyte
		size := 1048576
		data := make([]byte, size)
		n, _ := s.Read(data)
		fmt.Printf("received: %s\n", string(data[:n]))
	}
}
