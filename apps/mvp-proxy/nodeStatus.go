package main

import "encoding/json"

type MinaNodeStatusJson struct {
	IP                       string      `json:"node_ip_addr"`
	PeerId                   string      `json:"node_peer_id"`
	SyncStatus               string      `json:"sync_status"`
	ProtocolStateHash        string      `json:"protocol_state_hash"`
	GitCommit                string      `json:"git_commit"`
	UptimeMinutes            int64       `json:"uptime_minutes"`
	Peers                    []*MinaPeer `json:"peers"`
	BlockHashesAndTimestamps [][2]string `json:"k_block_hashes_and_timestamps"`
	// BanStatuses              []*MinaPeerBanStatus `json:"ban_statuses"`
}

type MinaPeer struct {
	Host   string `json:"host"`
	PeerId string `json:"peer_id"`
	Port   int    `json:"libp2p_port"`
}

type MinaPeerBanStatus struct {
	Trust  float64 `json:"trust"`
	Banned string  `json:"banned"`
}

func LoadMinaNodeStatusJson(data []byte) MinaNodeStatusJson {
	var s MinaNodeStatusJson
	json.Unmarshal(data, &s)
	return s
}
