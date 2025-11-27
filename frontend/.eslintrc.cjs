module.exports = {
    root: true,
    extends: ['plugin:svelte/recommended', 'prettier'],
    parser: '@babel/eslint-parser',
    overrides: [
        {
            files: ['*.svelte'],
            parser: 'svelte-eslint-parser',
        },
    ],
    parserOptions: {
        sourceType: 'module',
        ecmaVersion: 2020,
    },
    env: {
        browser: true,
        es2017: true,
        node: true,
    },
};
