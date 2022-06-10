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
            '--tw-prose-pre-bg': theme("colors.blue.50"),
            '--tw-prose-pre-border': theme("colors.blue.200"),
            '--tw-prose-pre-code': theme("colors.slate.900"),
            '--tw-prose-invert-pre-bg': theme("colors.slate.800"),
            '--tw-prose-invert-pre-border': theme("colors.slate.700"),
            '--tw-prose-invert-pre-code': theme("colors.slate-50"),

            pre: {
              borderColor: "var(--tw-prose-pre-border)",
              borderWidth: '1px',
              code: {
                padding: '0',
                border: '0'
              }
            },
            code: {
              backgroundColor: "var(--tw-prose-pre-bg)",
              fontWeight: 'normal',
              padding: '4px',
              borderRadius: '2px',
              borderWidth: '1px',
              borderColor: "var(--tw-prose-pre-border)",
              '&::before': {
                content: 'none !important',
              },
              '&::after': {
                content: 'none !important',
              },
            }
          }
        },

        invert: {
          css: {
            "--tw-prose-pre-border": "var(--tw-prose-invert-pre-border)"
          }
        }
      })
    }
  },
  plugins: [
    require('@tailwindcss/typography')
  ],
}
