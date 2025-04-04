/** @type {import("tailwindcss").Config} */
module.exports = {
  content: {
    files: ["*.html", "src/**/*.rs", "../ui/src/**/*.{html,rs}"],
    transform: {
      rs: (content) => {
        // First transform simple class:name cases
        // This handles all valid Tailwind class name characters
        let result = content.replace(
          /class:([a-zA-Z0-9_\-:\/\[\]\(\)\.\!]+)/g,
          " $1",
        );

        // Handle the class:name=expression case by capturing just the class name
        result = result.replace(
          /class:([a-zA-Z0-9_\-:\/\[\]\(\)\.\!]+)=/g,
          " $1 ",
        );

        return result;
      },
    },
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
