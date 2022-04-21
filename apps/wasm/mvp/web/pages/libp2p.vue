<template>
  <NavBar />
  <div class="container px-4 prose">
    <h2>Mina Web Node Demo using libp2p</h2>
    <p>wasm: {{ wasmStatus() }}</p>
    <p class="prose">
      Run
      <a
        href="https://github.com/ChainSafe/mina-rs/tree/mvp-proxy/apps/mvp-proxy"
      >proxy node</a>
      locally, then paste node address into below input box.
    </p>
    <p />
    <p>
      Local node address: <br>
      <input
        v-model="addr"
        class="addr form-input"
      > <br>
      <button
        v-if="wasmLoaded"
        class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white my-2"
        @click="connect"
      >
        Connect
      </button>
    </p>
    <div>
      Peers({{ getConnectedPeerCount() }} connected /
      {{ getPeerCount() }} total):
      <table>
        <tr>
          <th>id</th>
          <th>connected</th>
          <th>sync_status</th>
          <th>protocol_state_hash</th>
          <th>git_commit</th>
          <th>uptime_minutes</th>
        </tr>
        <tr
          v-for="id in peerKeys"
          :key="id"
        >
          <td>{{ id }}</td>
          <td>{{ peers[id].connected }}</td>
          <td>{{ peers[id].syncStatus }}</td>
          <td>{{ peers[id].protocolStateHash }}</td>
          <td>{{ peers[id].gitCommit }}</td>
          <td>{{ peers[id].uptimeMinutes }}</td>
        </tr>
      </table>
    </div>
  </div>
</template>

<script lang="ts">
import { initWasm } from "~/utils";
import { connect, set_event_emitter } from "~/pkg/wasm";
import { EventEmitter } from "events";
import { ConnectRequest } from "~/web/pb/requests";
import { PeerStatus } from "~/web/pb/messages";
import NavBar from "~/web/components/NavBar.vue";
import _ from "lodash";

export default {
  components: {
    NavBar,
  },
  data() {
    return {
      addr: "/ip4/127.0.0.1/tcp/23333/ws/p2p/12D3KooWFxXTHUv5Cpiq5gFJ8JUWW1LAwxcGJdaHt4S4GFZU8PgC",
      eventEmitter: new EventEmitter(),
      peers: {},
      peerKeys: [],
      wasmLoaded: false,
    };
  },
  created() {
    this.loadWasm();
  },
  mounted() {
    this.eventEmitter.on("log", (msg) => {
      console.log(`[log] ${msg}`);
    });
    this.eventEmitter.on("update", (msg) => {
      console.log(`[update] raw msg: ${msg}`);
      try {
        let ps = PeerStatus.decode(msg);
        console.log(`[update] decoded msg: ${ps}`);
        this.peers[ps.peerId] = ps;
        this.peerKeys = Object.keys(this.peers);
      } catch (e) {
        console.log(e);
        console.log(msg);
      }
    });
  },
  methods: {
    async loadWasm() {
      await initWasm();
      this.wasmLoaded = true;
    },
    wasmStatus() {
      return this.wasmLoaded ? "loaded" : "loading";
    },
    async connect() {
      if (!this.wasmLoaded) {
        return;
      }
      this.clear();
      set_event_emitter(this.eventEmitter);
      console.log(`[JS] Connecting to ${this.addr}`);
      let req = ConnectRequest.create();
      req.address = this.addr;
      try {
        await connect(ConnectRequest.encode(req).finish());
      } catch (e) {
        alert(e);
      }
    },
    clear() {
      this.peers = {};
      this.peerKeys = [];
    },
    getPeerCount() {
      return this.peerKeys.length;
    },
    getConnectedPeerCount() {
      return _.chain(this.peers)
        .filter((p) => p.connected)
        .size()
        .value();
    },
  },
};
</script>
<style lang="scss" scoped>
input.addr {
  width: 800px;
}
</style>
