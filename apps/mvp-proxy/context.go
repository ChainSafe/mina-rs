package main

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"time"

	"github.com/libp2p/go-libp2p-core/host"
	"github.com/libp2p/go-libp2p-core/peer"
	ma "github.com/multiformats/go-multiaddr"
)

type Context struct {
	Host          *host.Host
	PeerId2Status map[peer.ID]*PeerStatus
	NotifyFunc    *func(peer.ID, *PeerStatus)
}

func NewContext(host *host.Host) *Context {
	return &Context{
		Host:          host,
		PeerId2Status: make(map[peer.ID]*PeerStatus),
		NotifyFunc:    nil,
	}
}

type PeerStatus struct {
	Connected bool
	AddrInfo  *peer.AddrInfo
	Json      *MinaNodeStatusJson
	Timestamp time.Time
}

func (s *PeerStatus) ToLite(peerId peer.ID) PeerStatusLite {
	r := PeerStatusLite{
		PeerId:    peerId,
		Connected: s.Connected,
	}
	if s.Json != nil {
		r.SyncStatus = s.Json.SyncStatus
		r.ProtocolStateHash = s.Json.ProtocolStateHash
		r.GitCommit = s.Json.GitCommit
		r.UptimeMinutes = s.Json.UptimeMinutes
	}
	return r
}

type PeerStatusLite struct {
	Connected bool    `json:"connected"`
	PeerId    peer.ID `json:"peer_id"`

	SyncStatus        string `json:"sync_status"`
	ProtocolStateHash string `json:"protocol_state_hash"`
	GitCommit         string `json:"git_commit"`
	UptimeMinutes     int64  `json:"uptime_minutes"`
}

func (s *PeerStatusLite) ToJsonBytes() []byte {
	if b, err := json.Marshal(s); err == nil {
		return b
	}
	return []byte{}
}

func (s *PeerStatusLite) ToBase64EncodedJson() string {
	b := s.ToJsonBytes()
	return base64.RawStdEncoding.Strict().EncodeToString(b)
}

func (ctx *Context) UpdateStatus(addrInfo *peer.AddrInfo, connected bool, json *MinaNodeStatusJson) {
	if addrInfo == nil {
		return
	}
	status := &PeerStatus{AddrInfo: addrInfo, Connected: connected, Json: json, Timestamp: time.Now()}
	ctx.PeerId2Status[addrInfo.ID] = status
	if ctx.NotifyFunc != nil {
		(*ctx.NotifyFunc)(addrInfo.ID, status)
	}
}

func (ctx *Context) Loop() {
	now := time.Now()
	for _, v := range ctx.PeerId2Status {
		if v == nil || v.AddrInfo == nil {
			continue
		}
		if v.Timestamp.Add(time.Minute * 1).Before(now) {
			ctx.FetchNodeStatus(v.AddrInfo)
		}
		if v.Json != nil && v.Json.Peers != nil && len(v.Json.Peers) > 0 {
			for _, peerInfo := range v.Json.Peers {
				if peerInfo != nil {
					if mtaddr, err := ma.NewMultiaddr(fmt.Sprintf("/ip4/%s/tcp/%d/p2p/%s", peerInfo.Host, peerInfo.Port, peerInfo.PeerId)); err == nil {
						if minaPeerInfo, err := peer.AddrInfoFromP2pAddr(mtaddr); err == nil {
							ctx.FetchNodeStatus(minaPeerInfo)
						}
					}
				}
			}
		}
	}
}
