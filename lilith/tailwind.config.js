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
              color: theme("colors.slate.900"),
              borderColor: theme("colors.blue.200"),
              borderWidth: '1px',
              code: {
                padding: '0',
                border: '0'
              }
            },
            code: {
              backgroundColor: theme("colors.blue.50"),
              fontWeight: 'normal',
              padding: '4px',
              borderRadius: '2px',
              borderWidth: '1px',
              borderColor: theme("colors.blue.200"),
              '&::before': {
                content: 'none !important',
              },
              '&::after': {
                content: 'none !important',
              },
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
