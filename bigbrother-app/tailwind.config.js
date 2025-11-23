/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#3B82F6',
        purple: {
          DEFAULT: '#A855F7',
          light: '#C084FC',
          dark: '#7E22CE',
        },
        orange: {
          DEFAULT: '#FB923C',
          light: '#FDBA74',
          dark: '#EA580C',
        },
        success: '#10B981',
        warning: '#F59E0B',
        danger: '#EF4444',
      },
    },
  },
  plugins: [],
}

