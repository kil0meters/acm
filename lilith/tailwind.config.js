module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html"
  ],
  theme: {
    extend: {
      gridTemplateRows: {
        'min-full': 'min-content minmax(0,1fr)',
        'full-min': 'minmax(0,1fr) auto'
      },
      gridTemplateColumns: {
        'full': 'minmax(0,1fr)',
        'min-full': 'min-content minmax(0,1fr)'
      },
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0' },
          '50%': { opacity: '0' },
          '100%': { opacity: '1' },
        }
      },
      animation: {
        'fade-in': 'fade-in 1s ease-in-out'
      },
      typography: theme => ({
        neutral: {
          css: {
            pre: {
              backgroundColor: theme("colors.blue.50"),
              color: theme("colors.slate.900")
            }
          }
        }
      })
    }
  },
  plugins: [
    require('@tailwindcss/typography')
  ],
}
