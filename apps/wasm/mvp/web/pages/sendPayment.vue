<template>
    <NavBar />
    <div class="container px-4 prose min-w-full">
        <h2>Mina Send Payment Demo</h2>
        <p>wasm: {{ wasmStatus() }}</p>
        <p>
          <span>Public Key: </span><input
            v-model="publicKey"
            type="text">
        </p>
        <p>
            <button class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white" @click="accountInfo">Get Account Info</button>
        </p>
        <p>{{ accountDetails }}</p>
        <p>
            <button class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white" @click="unlockAccount">Unlock Account</button>
        </p>
        <p>{{ isAccountLocked }}</p>
        <h5>Payment Details:</h5>
        <textarea v-model="paymentStr"></textarea>
        <p>
            <button class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white" @click="sendSignedPayment">Send Payment</button>
        </p>
        <p>{{ paymentId }}</p>
        <p>
            <button class="bg-sky-500 hover:bg-sky-700 px-5 py-2 text-sm leading-5 rounded-full font-semibold text-white" @click="lockAccount">Lock Account</button>
        </p>
        <p>{{ isAccountLocked }}</p>
    </div>
</template>
<script lang="ts">
import NavBar from "~/web/components/NavBar.vue";
import wasmUrl from "raw:./../../pkg/wasm_bg.wasm";
import init, { accountInfo, lockAccount, unlockAccount, sendPayment} from "~/pkg/wasm";

export default {
    components: {
        NavBar,
    },
    data() {
        return {
            wasmLoaded: false,
            paymentStr: JSON.stringify({
                to: 'B62qncQVG8FULDcgM82Yg659yiwESbLWQ39ALVakbCktqtEQwmpRk6n',
                from: '',
                fee: 1000000,
                amount: 1,
                validUntil: 0xFFFFFFFF,
            }, null, 2),
            accountDetails: '',
            isAccountLocked: '',
            paymentId: '',
        };
    },
    async created() {
        await init(await fetch(wasmUrl));
        this.wasmLoaded = true;
    },
    methods: {
        wasmStatus() {
            return this.wasmLoaded ? "loaded" : "loading";
        },
        async accountInfo() {
            try {
                if (!this.publicKey) {
                    alert('Input PublicKey for Account!!');
                    return;
                }
                this.accountDetails = await accountInfo(this.publicKey);
            } catch (e) {
                this.accountDetails = `${e}`;
            }
        },
        async lockAccount() {
            try {
                if (!this.publicKey) {
                    this.publicKey = prompt('Input PublicKey for Account!!');
                }
                this.isAccountLocked = await lockAccount(this.publicKey, this.password);
                this.password = '';
            } catch (e) {
                this.isAccountLocked = `${e}`;
            }
        },
        async unlockAccount() {
            try {
                if (!this.publicKey) {
                    alert('Input PublicKey for Account!!');
                    return;
                }
                if (!this.password) {
                    this.password = prompt('Input Password for Unlocking Account!!');
                }
                this.isAccountLocked = await unlockAccount(this.publicKey, this.password);
            } catch (e) {
                this.isAccountLocked = `${e}`;
            }
        },
        async sendSignedPayment() {
            try {
                if (!this.publicKey) {
                    alert('Input PublicKey for Account!!');
                    return;
                }
                const payment = JSON.parse(this.paymentStr);
                payment.from = this.publicKey;
                this.paymentStr = JSON.stringify(payment, null, 2);
                this.paymentId = await sendPayment(payment);
            } catch (e) {
                this.paymentId = `${e}`;
            }
        },
    }
};
</script>
<style lang="scss" scoped>
textarea {
    width: 75%;
    height: 200px;
}
</style>
