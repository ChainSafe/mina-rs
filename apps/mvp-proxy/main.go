package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"log"
	"sync"
	"time"

	"github.com/libp2p/go-libp2p-core/network"
	"github.com/libp2p/go-libp2p-core/peer"
	"github.com/libp2p/go-libp2p/p2p/protocol/identify"

	ma "github.com/multiformats/go-multiaddr"
)

func run() {
	var port int
	flag.IntVar(&port, "port", 23333, "websocket port, default is 23333")
	flag.Parse()
	proxyHost, err := createWsProxyHost(port)
	if err != nil {
		log.Printf("Failed to create proxyHost: %v", err)
		return
	}
	hostAddrStr := fmt.Sprintf("%s/p2p/%s", proxyHost.Addrs()[0], proxyHost.ID())
	ctx := NewContext(&proxyHost)
	proxyHost.SetStreamHandler("/mina-proxy/node-status", func(s network.Stream) {
		rw := bufio.NewReadWriter(bufio.NewReader(s), bufio.NewWriter(s))
		mutex := &sync.RWMutex{}
		var notify func(peerId peer.ID, status *PeerStatus) = func(peerId peer.ID, status *PeerStatus) {
			fmt.Printf("Updating peer status for %s ... ", peerId)
			statusLite := status.ToLite(peerId)
			j := statusLite.ToBase64EncodedJson() + "\n"
			mutex.Lock()
			defer mutex.Unlock()
			fmt.Println(len(j))
			if _, err := rw.WriteString(j); err == nil {
				rw.Flush()
			}
			// mutex.Unlock()
		}
		ctx.NotifyFunc = &notify
		for k, v := range ctx.PeerId2Status {
			notify(k, v)
		}
		// Ping
		go func() {
			for {
				{
					mutex.Lock()
					// defer mutex.Unlock()
					if _, err := rw.WriteString("\n"); err != nil {
						mutex.Unlock()
						break
					}
					rw.Flush()
					mutex.Unlock()
				}
				time.Sleep(time.Second * 10)
			}
		}()
	})
	// mypubsub, err := pubsub.NewGossipSub(context.Background(), proxyHost)

	// log.Printf("proxyHost: %s %s\n", proxyHost.Addrs(), proxyHost.ID())
	log.Printf("proxyHost: %s\n", hostAddrStr)

	peerChan := initMDNS(context.Background(), proxyHost, mdnsRendezvousString)
	idService, _ := identify.NewIDService(proxyHost)
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
}

func (ctx *Context) FetchNodeStatus(addrInfo *peer.AddrInfo) {
	host := *ctx.Host
	c := host.Network().Connectedness(addrInfo.ID)
	if c != network.Connected {
		if err := host.Connect(context.Background(), *addrInfo); err != nil {
			log.Printf("Failed to connect to mina node %s: %v", addrInfo.ID, err)
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
	run()
	wg := sync.WaitGroup{}
	wg.Add(1)
	wg.Wait()
}
