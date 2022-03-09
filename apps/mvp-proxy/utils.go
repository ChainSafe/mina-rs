package main

import (
	"context"
	"crypto/rand"
	"fmt"

	"github.com/libp2p/go-libp2p"
	"github.com/libp2p/go-libp2p-core/crypto"
	"github.com/libp2p/go-libp2p-core/host"
	"github.com/libp2p/go-libp2p-core/network"
	ma "github.com/multiformats/go-multiaddr"
)

func createWsProxyHost(port int) (host host.Host, err error) {
	listenAddrWs, _ := ma.NewMultiaddr(fmt.Sprintf("/ip4/127.0.0.1/tcp/%d/ws", port))
	listenAddrs := libp2p.ListenAddrs(listenAddrWs)
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
