import { themes as prismThemes } from "prism-react-renderer";
import type { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

const config: Config = {
  title: "Holt",
  tagline: "A UI toolkit for Leptos with behavior/presentation separation",
  favicon: "img/favicon.ico",

  url: "https://holt.rs",
  baseUrl: "/",

  organizationName: "aonyx",
  projectName: "holt",

  onBrokenLinks: "throw",

  markdown: {
    preprocessor: ({ filePath, fileContent }) => fileContent,
    hooks: {
      onBrokenMarkdownLinks: "warn",
    },
  },

  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: "./sidebars.ts",
          editUrl: "https://github.com/aonyx-labs/holt/tree/main/docs/",
        },
        blog: false,
        theme: {
          customCss: "./src/css/custom.css",
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: "img/holt-social-card.png",
    navbar: {
      logo: {
        alt: "Holt",
        src: "img/logo.svg",
      },
      items: [
        {
          type: "docSidebar",
          sidebarId: "docsSidebar",
          position: "left",
          label: "Docs",
        },
        {
          type: "html",
          position: "left",
          value: '<a href="/kit/" class="navbar__item navbar__link">Kit</a>',
        },
        {
          href: "https://github.com/aonyx-labs/holt",
          label: "GitHub",
          position: "right",
        },
      ],
    },
    footer: {
      style: "dark",
      links: [
        {
          title: "Learn",
          items: [
            {
              label: "Tutorials",
              to: "/docs/tutorials",
            },
            {
              label: "How-to Guides",
              to: "/docs/guides",
            },
          ],
        },
        {
          title: "Understand",
          items: [
            {
              label: "Explanation",
              to: "/docs/explanation",
            },
            {
              label: "Reference",
              to: "/docs/reference",
            },
          ],
        },
        {
          title: "Community",
          items: [
            {
              label: "GitHub",
              href: "https://github.com/aonyx-labs/holt",
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Aonyx Labs. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ["rust", "toml", "bash"],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
