// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

const indexPageUrl = chrome.runtime.getURL("index.html")

if (chrome.action) {
    // Manifest v3
    chrome.action.onClicked.addListener(function (_tab) {
        chrome.tabs.create({ url: indexPageUrl });
    });
} else if (chrome.browserAction) {
    // Manifest v2
    chrome.browserAction.onClicked.addListener(function (_tab) {
        chrome.tabs.create({ url: indexPageUrl });
    });
}

chrome.runtime.onInstalled.addListener((_reason) => {
    chrome.tabs.create({
        url: indexPageUrl
    });
});
