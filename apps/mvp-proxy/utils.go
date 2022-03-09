package main

import (
	"context"
	"crypto/rand"

	"github.com/libp2p/go-libp2p"
	relay "github.com/libp2p/go-libp2p-circuit"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/host"
	"github.com/libp2p/go-libp2p-core/network"
	"github.com/libp2p/go-libp2p/config"
	ma "github.com/multiformats/go-multiaddr"
)

func createRelayHost(isRelayNode bool, listenEnabled bool) (host host.Host, err error) {
	var relayOps config.Option
	if isRelayNode {
		_ = relay.OptHop
		relayOps = libp2p.EnableRelay(relay.OptHop)
	} else {
		relayOps = libp2p.EnableRelay()
	}
	var listenAddrs config.Option
	if listenEnabled {
		// listenAddr, _ := ma.NewMultiaddr("/ip4/127.0.0.1/tcp/0")
		listenAddrWs, _ := ma.NewMultiaddr("/ip4/127.0.0.1/tcp/23333/ws")
		listenAddrs = libp2p.ListenAddrs(listenAddrWs)
	} else {
		listenAddrs = libp2p.ListenAddrs()
	}
	r := rand.Reader
	prvKey, _, err := crypto.GenerateKeyPairWithReader(crypto.Ed25519, 2048, r)
	if err != nil {
		panic(err)
	}
	// ps, err := pstoremem.NewPeerstore()
	host, err = libp2p.New(
		context.Background(),
		libp2p.Identity(prvKey),
		listenAddrs,
		relayOps,
		muxer,
		userAgent,
		privateNetwork,
		// libp2p.Peerstore(ps),
	)
	return
}

func readBytes(s network.Stream) []byte {
	data := make([]byte, BUFFER_SIZE)
	n, _ := s.Read(data)
	if n < len(data) {
		return data[:n]
	} else {
		buffer := make([]byte, BUFFER_SIZE)
		for {
			n, _ := s.Read(buffer)
			if n == 0 {
				break
			} else if n < len(buffer) {
				data = append(data, buffer[:n]...)
				break
			} else {
				data = append(data, buffer...)
			}
		}
		return data
	}
}
