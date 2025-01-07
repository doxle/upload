/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: "class", // Use class based theming
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      borderWidth: {
        1: "1px", // Add custom border width
      },

      typography: {
        DEFAULT: {
          css: {
            pre: {
              backgroundColor: "var(--bg-color)",
            },
          },
        },
      },
      borderWidth: {
        0.5: "0.5px",
      },
      fontFamily: {
        // inter: ["Inter", "sans-serif"],
        // montserrat: ["Montserrat", "sans-serif"],
        // lexend: ["Lexend", "sans-serif"],

        helvetica: ['"Helvetica Neue"', "Arial", "sans-serif"],
      },
    },
  },
  plugins: [],
};
