<template>
  <div><h2>Mina Web Node Demo</h2></div>
  <p>
    Run
    <a href="https://github.com/ChainSafe/mina-rs/tree/mvp-proxy/apps/mvp-proxy">proxy node</a>
    locally, then paste node address into below input box.
  </p>
  <p />
  <p>
    Local node address: <br>
    <input
      v-model="addr"
      class="addr"
    > <br>
    <button @click="connect">
      Connect
    </button>
  </p>
  <div>
    Peers:
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
        <td>{{ peers[id].sync_status }}</td>
        <td>{{ peers[id].protocol_state_hash }}</td>
        <td>{{ peers[id].git_commit }}</td>
        <td>{{ peers[id].uptime_minutes }}</td>
      </tr>
    </table>
  </div>
</template>

<script lang="ts">
import { connect, set_event_emitter } from "./../pkg/wasm";
import { EventEmitter } from "events";

export default {
  data() {
    return {
      addr: "/ip4/127.0.0.1/tcp/23333/ws/p2p/12D3KooWFxXTHUv5Cpiq5gFJ8JUWW1LAwxcGJdaHt4S4GFZU8PgC",
      eventEmitter: new EventEmitter(),
      peers: {},
      peerKeys: [],
    };
  },
  mounted() {
    this.eventEmitter.on("update", (msg) => {
      console.log(`eventEmitter onupdate: ${msg}`);
      try {
        const o = JSON.parse(msg);
        // console.log(o);
        this.peers[o["peer_id"]] = o;
        this.peerKeys = Object.keys(this.peers);
      } catch (e) {
        console.log(e);
        console.log(msg);
      }
    });
  },
  methods: {
    async connect() {
      this.clear();
      set_event_emitter(this.eventEmitter);
      console.log(`Connecting to ${this.addr}`);
      await connect(this.addr);
    },
    clear() {
      this.peers = {};
      this.peerKeys = [];
    },
  },
};
</script>
<style lang="scss" scoped>
input.addr {
  width: 800px;
}
</style>
