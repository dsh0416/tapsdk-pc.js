import { defineConfig } from 'vitepress';

// Base path for GitHub Pages deployment
// Set VITEPRESS_BASE env var to override (e.g., '/tapsdk-pc-js/')
const base = process.env.VITEPRESS_BASE || '/';

export default defineConfig({
  title: 'TapTap PC SDK',
  description: 'Node.js bindings for TapTap PC SDK',
  base,

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: `${base}logo.svg` }],
  ],

  themeConfig: {
    logo: '/logo.svg',

    nav: [
      { text: 'Guide', link: '/getting-started' },
      { text: 'API', link: '/api/' },
      { text: 'Examples', link: '/examples' },
    ],

    sidebar: {
      '/': [
        {
          text: 'Introduction',
          items: [
            { text: 'Getting Started', link: '/getting-started' },
            { text: 'Examples', link: '/examples' },
          ],
        },
        {
          text: 'API Reference',
          items: [
            { text: 'Overview', link: '/api/' },
            { text: 'TapSdk', link: '/api/tapsdk' },
            { text: 'CloudSave', link: '/api/cloudsave' },
            { text: 'Events', link: '/api/events' },
            { text: 'Types', link: '/api/types' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/dsh0416/tapsdk-pc.js' },
    ],

    editLink: {
      pattern: 'https://github.com/dsh0416/tapsdk-pc.js/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present',
    },

    search: {
      provider: 'local',
    },

    outline: {
      level: [2, 3],
    },
  },
});
