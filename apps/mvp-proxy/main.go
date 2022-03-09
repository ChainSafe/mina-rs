package main

import (
	"bufio"
	"context"
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/libp2p/go-libp2p-core/network"
	"github.com/libp2p/go-libp2p-core/peer"
	"github.com/libp2p/go-libp2p/p2p/protocol/identify"
	mplex "github.com/libp2p/go-mplex"
	ma "github.com/multiformats/go-multiaddr"
	// pubsub "github.com/libp2p/go-libp2p-pubsub"
)

func run() {
	relayHost, err := createRelayHost(true, true)
	if err != nil {
		log.Printf("Failed to create relayHost: %v", err)
		return
	}
	hostAddrStr := fmt.Sprintf("%s/p2p/%s", relayHost.Addrs()[0], relayHost.ID())
	ctx := NewContext(&relayHost)
	relayHost.SetStreamHandler("/webnode", func(s network.Stream) {
		rw := bufio.NewReadWriter(bufio.NewReader(s), bufio.NewWriter(s))
		mutex := &sync.RWMutex{}
		{
			mutex.Lock()
			// defer mutex.Unlock()
			rw.WriteString("Hello\nWorld\n")
			rw.Flush()
			mutex.Unlock()
		}
		var f func(peerId peer.ID, status *PeerStatus) = func(peerId peer.ID, status *PeerStatus) {
			fmt.Printf("Updating peer status for %s ... ", peerId)
			statusLite := status.ToLite(peerId)
			j := statusLite.ToBase64EncodedJson() + "\n"
			mutex.Lock()
			// defer mutex.Unlock()
			fmt.Println(len(j))
			if _, err := rw.WriteString(j); err == nil {
				rw.Flush()
			}
			mutex.Unlock()
		}
		ctx.NotifyFunc = &f
		for k, v := range ctx.PeerId2Status {
			f(k, v)
		}
		// Ping
		go func() {
			for {
				{
					mutex.Lock()
					// defer mutex.Unlock()
					if _, err := rw.WriteString("\n"); err != nil {
						break
					}
					rw.Flush()
					mutex.Unlock()
				}
				time.Sleep(time.Second * 10)
			}
		}()
	})
	// mypubsub, err := pubsub.NewGossipSub(context.Background(), relayHost)

	log.Printf("relayHost: %s %s\n", relayHost.Addrs(), relayHost.ID())
	log.Printf("relayHost: %s\n", hostAddrStr)

	peerChan := initMDNS(context.Background(), relayHost, mdnsRendezvousString)
	idService, _ := identify.NewIDService(relayHost)
	_ = idService
	go func() {
		peer := <-peerChan // will block untill we discover a peer
		fmt.Println("[MDNS] Found peer: ", peer)
	}()
	go func() {
		for {
			fmt.Printf("[%s] Context peers: %d\n", hostAddrStr, len(ctx.PeerId2Status))
			time.Sleep(time.Second * 10)
		}
	}()

	for _, seed := range SEEDS {
		if mtaddr, err := ma.NewMultiaddr(seed); err == nil {
			if minaPeerInfo, err := peer.AddrInfoFromP2pAddr(mtaddr); err == nil {
				ctx.FetchNodeStatus(minaPeerInfo)
			}
		}
	}

	for {
		ctx.Loop()
		time.Sleep(time.Second * 10)
	}

	log.Printf("relayHost: %s\n", hostAddrStr)
}

func (ctx *Context) FetchNodeStatus(addrInfo *peer.AddrInfo) {
	host := *ctx.Host
	c := host.Network().Connectedness(addrInfo.ID)
	if c != network.Connected {
		if err := host.Connect(context.Background(), *addrInfo); err != nil {
			log.Printf("Failed to connect relayHost to mina: %v", err)
			ctx.UpdateStatus(addrInfo, false, nil)
			return
		}
	}
	s, err := host.NewStream(context.Background(), addrInfo.ID, "/mina/node-status")
	if err != nil {
		log.Println("huh, this should have worked: ", err)
		ctx.UpdateStatus(addrInfo, false, nil)
		return
	}
	time.Sleep(time.Second * 1)
	data := readBytes(s)
	json := LoadMinaNodeStatusJson(data)
	ctx.UpdateStatus(addrInfo, true, &json)
}

func main() {
	mplex.MaxMessageSize = 1 << 30
	run()
	wg := sync.WaitGroup{}
	wg.Add(1)
	wg.Wait()
}
