import type { SidebarsConfig } from "@docusaurus/plugin-content-docs";

const sidebars: SidebarsConfig = {
  docsSidebar: [
    {
      type: "category",
      label: "Tutorials",
      link: {
        type: "doc",
        id: "tutorials/index",
      },
      items: ["tutorials/first-storybook", "tutorials/snapshot-testing"],
    },
    {
      type: "category",
      label: "How-to Guides",
      link: {
        type: "doc",
        id: "guides/index",
      },
      items: [
        "guides/customize-styling",
        "guides/dark-mode",
        "guides/snapshot-testing",
      ],
    },
    {
      type: "category",
      label: "Explanation",
      link: {
        type: "doc",
        id: "explanation/index",
      },
      items: [
        "explanation/behavior-presentation",
        "explanation/styling",
        "explanation/architecture",
        "explanation/snapshot-testing",
      ],
    },
    {
      type: "category",
      label: "Reference",
      link: {
        type: "doc",
        id: "reference/index",
      },
      items: ["reference/cli", "reference/story-macro"],
    },
  ],
};

export default sidebars;
