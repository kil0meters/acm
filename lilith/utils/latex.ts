import renderMathInElement from "katex/contrib/auto-render"

export default function renderLatex(element: HTMLElement) {
  renderMathInElement(element, {
    // customised options
    // • auto-render specific keys, e.g.:
    delimiters: [
      { left: '$$', right: '$$', display: true },
      { left: '$', right: '$', display: false },
      { left: '\\(', right: '\\)', display: false },
      { left: '\\[', right: '\\]', display: true }
    ],
    // • rendering keys, e.g.:
    throwOnError: false
  })
}
