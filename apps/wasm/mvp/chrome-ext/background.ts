const indexPageUrl = chrome.runtime.getURL("index.html")

chrome.action.onClicked.addListener(function (_tab) {
    chrome.tabs.create({ url: indexPageUrl });
});

chrome.runtime.onInstalled.addListener((_reason) => {
    chrome.tabs.create({
        url: indexPageUrl
    });
});
