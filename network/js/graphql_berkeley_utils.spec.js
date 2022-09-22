// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

const { fetch_best_chain_json } = require("./graphql_berkeley_utils.js");

jest.setTimeout(30000);

test("dummy", () => {});

test("fetch_best_chain_json", async () => {
	if (isCI()) {
		return;
	}

	const endpoint = "http://localhost:3085/graphql";
	const json = await fetch_best_chain_json(endpoint, [
		"B62qiy32p8kAKnny8ZFwoMhYpBppM1DWVCqAPBYNcXnsAHhnfAAuXgg",
		"B62qknCv9QdyAvt4Te58oo3nrZTacpEcjpJg1MV61r94h5rDPDUyPP8",
	]);
	console.log(JSON.stringify(json, null, 2));
});

// https://docs.github.com/en/actions/learn-github-actions/environment-variables#default-environment-variables
function isCI() {
	return !!process.env.CI;
}
