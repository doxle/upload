/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: "class", // Use class based theming
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      backgroundImage: {
          grid: "linear-gradient(to right, rgba(213, 218, 248, 0.4) 1px, transparent 1px), linear-gradient(to bottom, rgba(213, 218, 248, 0.4) 1px, transparent 1px)",
      },
      backgroundSize: {
        grid: "40px 40px",
        sm: "20px 20px",
        md: "40px 40px",
        lg: "50px 50px",
      },
      borderWidth: {
        1: "1px",
      },
      letterSpacing: {
        tightest: '-0.25em', // Adjust as needed (e.g., for -4px)
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
      fontWeight: {
              thin: '100',
              extralight: '200',
              light: '300',
              bold: '700',
      },
    },
  },
  plugins: [
      function({ addUtilities }) {
        const newUtilities = {
          '.antialiased': {
            '-webkit-font-smoothing': 'antialiased',
            '-moz-osx-font-smoothing': 'grayscale',
          },
          '.subpixel-antialiased': {
            '-webkit-font-smoothing': 'subpixel-antialiased',
          },
        };

        addUtilities(newUtilities, ['responsive', 'hover']);
      },
    ],
};
