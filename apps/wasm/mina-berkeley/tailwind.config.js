// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

module.exports = {
    content: [
        "./web/**/*.{html,vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),],
}
